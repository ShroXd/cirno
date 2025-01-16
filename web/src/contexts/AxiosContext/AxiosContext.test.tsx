import { ReactNode, useContext } from 'react'

import { render, renderHook } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import {
  EventBusContext,
  EventBusContextProps,
} from '../EventBusContext/EventBusContext'
import { EventHandler, EventType } from '../EventBusContext/types'
import { AxiosContext, AxiosProvider } from './AxiosContext'

const mockAxiosInstance = {
  interceptors: {
    request: {
      use: vi.fn(),
      headers: {},
    },
    response: {
      use: vi.fn(),
    },
  },
}

vi.mock('axios', () => ({
  default: {
    create: () => mockAxiosInstance,
  },
}))

interface TestComponentProps extends Partial<EventBusContextProps> {
  children?: ReactNode
}

describe('AxiosContext', async () => {
  const wrapper = ({
    emitEvent = vi.fn(),
    onEvent = vi.fn(),
    offEvent = vi.fn(),
    children,
  }: TestComponentProps) => (
    <EventBusContext.Provider value={{ emitEvent, onEvent, offEvent }}>
      <AxiosProvider>{children}</AxiosProvider>
    </EventBusContext.Provider>
  )

  it('should provide context to child components', () => {
    const { result } = renderHook(() => useContext(AxiosContext), {
      wrapper,
    })

    expect(result.current).toBeDefined()
    expect(result.current?.axiosInstance).toBeDefined()
  })

  it('should set the client key in the request headers when event is triggered', () => {
    const mockRequestConfig = { headers: {} as Record<string, string> }
    const mockUse = vi.fn(callback => callback(mockRequestConfig))

    mockAxiosInstance.interceptors.request.use = mockUse

    let cacheEventName: EventType = 'Unknown' as EventType
    let cacheHandler: EventHandler<'RegisterClient'> = () => {
      throw new Error('Handler not initialized')
    }

    const onEventMock = vi.fn((eventName, handlerFn) => {
      cacheEventName = eventName
      cacheHandler = handlerFn
    })

    render(
      wrapper({
        onEvent: onEventMock,
      })
    )

    expect(cacheEventName).toBe('RegisterClient')
    expect(onEventMock).toHaveBeenCalledTimes(1)
    expect(onEventMock).toHaveBeenCalledWith(
      'RegisterClient',
      expect.any(Function)
    )

    cacheHandler({ clientKey: '123' })

    expect(mockUse).toHaveBeenCalledTimes(1)
    expect(mockUse).toHaveBeenCalledWith(expect.any(Function))

    expect(mockRequestConfig.headers?.['X-WS-CLIENT-KEY']).toBe('123')
  })
})
