import { ReactNode, useContext } from 'react'

import { renderHook } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import { EventBusContext } from '../EventBusContext/EventBusContext'
import { AxiosContext, AxiosProvider } from './AxiosContext'

describe('AxiosContext', async () => {
  const wrapper = ({ children }: { children: ReactNode }) => {
    return (
      <EventBusContext.Provider
        value={{ emitEvent: vi.fn(), onEvent: vi.fn(), offEvent: vi.fn() }}
      >
        <AxiosProvider>{children}</AxiosProvider>
      </EventBusContext.Provider>
    )
  }

  it('should provide context to child components', () => {
    const { result } = renderHook(() => useContext(AxiosContext), {
      wrapper,
    })

    expect(result.current).toBeDefined()
    expect(result.current?.axiosInstance).toBeDefined()
  })
})
