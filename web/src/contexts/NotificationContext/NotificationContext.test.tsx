import { ReactNode, useContext } from 'react'

import { act, renderHook } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'

import {
  NotificationContext,
  NotificationProvider,
} from './NotificationContext'

describe('NotificationContext', () => {
  vi.spyOn(Math, 'random').mockReturnValue(0.123456789)

  const wrapper = ({ children }: { children: ReactNode }) => (
    <NotificationProvider>{children}</NotificationProvider>
  )

  afterEach(() => {
    vi.restoreAllMocks()
  })

  it('should provide context to child components', () => {
    const { result } = renderHook(() => useContext(NotificationContext), {
      wrapper,
    })

    expect(result.current).toBeDefined()
    expect(result.current?.addNotification).toBeDefined()
    expect(result.current?.removeNotification).toBeDefined()
  })

  it('should be able to add a notification', () => {
    const { result } = renderHook(() => useContext(NotificationContext), {
      wrapper,
    })

    const notification = {
      title: 'Test Notification',
      message: 'This is a test notification',
    }

    let id = ''
    act(() => {
      id = result.current?.addNotification(notification)
    })

    expect(id).toBeDefined()
    expect(result.current?.getAllNotifications()).toHaveLength(1)
    expect(result.current?.getAllNotifications()[0]).toEqual({
      ...notification,
      id,
    })
  })
})
