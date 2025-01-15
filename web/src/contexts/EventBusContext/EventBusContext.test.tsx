import { useContext } from 'react'

import { renderHook } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import { EventBusContext, EventBusProvider } from './EventBusContext'
import { EventType } from './types'

describe('EventBusContext', () => {
  const wrapper = ({ children }: { children: React.ReactNode }) => (
    <EventBusProvider>{children}</EventBusProvider>
  )

  it('should provide context to child components', () => {
    const { result } = renderHook(() => useContext(EventBusContext), {
      wrapper,
    })

    expect(result.current).toBeDefined()
    expect(result.current?.emitEvent).toBeDefined()
    expect(result.current?.onEvent).toBeDefined()
    expect(result.current?.offEvent).toBeDefined()
  })

  it('should memoize event bus instance', () => {
    const { result, rerender } = renderHook(() => useContext(EventBusContext), {
      wrapper,
    })
    const firstResult = result.current

    rerender()

    expect(result.current).toBe(firstResult)
  })

  it('should handle event subscription and emission', () => {
    const { result } = renderHook(() => useContext(EventBusContext), {
      wrapper,
    })
    const event = 'testEvent' as EventType
    const payload = { test: 'test' }

    const { emitEvent, onEvent } = result.current!
    const eventHandler = vi.fn()

    onEvent(event, eventHandler)
    emitEvent({ event, payload })

    expect(eventHandler).toHaveBeenCalledTimes(1)
    expect(eventHandler).toHaveBeenCalledWith(payload)
  })

  it('should handle event unsubscription', () => {
    const { result } = renderHook(() => useContext(EventBusContext), {
      wrapper,
    })
    const event = 'testEvent' as EventType
    const payload = { test: 'test' }

    const { emitEvent, onEvent, offEvent } = result.current!
    const eventHandler = vi.fn()

    onEvent(event, eventHandler)
    offEvent(event, eventHandler)
    emitEvent({ event, payload })

    expect(eventHandler).not.toHaveBeenCalled()
  })

  it('should handle multiple event subscriptions', () => {
    const { result } = renderHook(() => useContext(EventBusContext), {
      wrapper,
    })
    const event1 = 'testEvent1' as EventType
    const payload1 = { test: 'test1' }
    const event2 = 'testEvent2' as EventType
    const payload2 = { test: 'test2' }

    // event1 -> handler1, handler2
    // event2 -> handler3
    const { emitEvent, onEvent } = result.current!
    const eventHandler1 = vi.fn()
    const eventHandler2 = vi.fn()
    const eventHandler3 = vi.fn()

    onEvent(event1, eventHandler1)
    onEvent(event1, eventHandler2)
    onEvent(event2, eventHandler3)
    emitEvent({ event: event1, payload: payload1 })
    emitEvent({ event: event2, payload: payload2 })

    expect(eventHandler1).toHaveBeenCalledTimes(1)
    expect(eventHandler1).toHaveBeenCalledWith(payload1)
    expect(eventHandler2).toHaveBeenCalledTimes(1)
    expect(eventHandler2).toHaveBeenCalledWith(payload1)
    expect(eventHandler3).toHaveBeenCalledTimes(1)
    expect(eventHandler3).toHaveBeenCalledWith(payload2)
  })
})
