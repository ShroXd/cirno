import { describe, it, expect, vi, beforeEach } from 'vitest'
import { createEventBus } from './eventBus'
import { EventBusType } from './types'

describe('EventBus', () => {
  let eventBus: EventBusType

  beforeEach(() => {
    eventBus = createEventBus()
  })

  it('should work correctly', () => {
    const handler = vi.fn()
    const payload = { clientKey: '123' }

    eventBus.on('RegisterClient', handler)
    eventBus.emit('RegisterClient', payload)

    expect(handler).toHaveBeenCalledWith(payload)
  })

  it('should allow multiple handlers for the same event', () => {
    const handler1 = vi.fn()
    const handler2 = vi.fn()

    eventBus.on('RegisterClient', handler1)
    eventBus.on('RegisterClient', handler2)

    const payload = { clientKey: '123' }
    eventBus.emit('RegisterClient', payload)

    expect(handler1).toHaveBeenCalledWith(payload)
    expect(handler2).toHaveBeenCalledWith(payload)
  })

  it('should unsubscribe from events with off() method', () => {
    const handler = vi.fn()
    const payload = { clientKey: '123' }

    eventBus.on('RegisterClient', handler)
    eventBus.off('RegisterClient', handler)
    eventBus.emit('RegisterClient', payload)

    expect(handler).not.toHaveBeenCalled()
  })

  it('should only unsubscribe the specified handler', () => {
    const handler1 = vi.fn()
    const handler2 = vi.fn()

    eventBus.on('RegisterClient', handler1)
    eventBus.on('RegisterClient', handler2)
    eventBus.off('RegisterClient', handler1)

    const payload = { clientKey: '123' }
    eventBus.emit('RegisterClient', payload)

    expect(handler1).not.toHaveBeenCalled()
    expect(handler2).toHaveBeenCalledWith(payload)
  })

  it('should not fail when emitting event with no handlers', () => {
    expect(() => {
      eventBus.emit('RegisterClient', { clientKey: '123' })
    }).not.toThrow()
  })

  it('should not fail when unsubscribing non-existent handler', () => {
    const handler = vi.fn()
    expect(() => {
      eventBus.off('RegisterClient', handler)
    }).not.toThrow()
  })
})
