import { describe, it, expect, vi, beforeEach } from 'vitest'
import { createEventBus, EventBusType, VideoPlayerEventType } from './eventBus'

describe('EventBus', () => {
  let eventBus: EventBusType

  beforeEach(() => {
    eventBus = createEventBus()
  })

  it('should work correctly', () => {
    const handler = vi.fn()
    const payload = { id: '123' }

    eventBus.on(VideoPlayerEventType.Play, handler)
    eventBus.emit(VideoPlayerEventType.Play, payload)

    expect(handler).toHaveBeenCalledWith(payload)
  })

  it('should allow multiple handlers for the same event', () => {
    const handler1 = vi.fn()
    const handler2 = vi.fn()

    eventBus.on(VideoPlayerEventType.Play, handler1)
    eventBus.on(VideoPlayerEventType.Play, handler2)

    const payload = { id: '123' }
    eventBus.emit(VideoPlayerEventType.Play, payload)

    expect(handler1).toHaveBeenCalledWith(payload)
    expect(handler2).toHaveBeenCalledWith(payload)
  })

  it('should unsubscribe from events with off() method', () => {
    const handler = vi.fn()
    const payload = { id: '123' }

    eventBus.on(VideoPlayerEventType.Play, handler)
    eventBus.off(VideoPlayerEventType.Play, handler)
    eventBus.emit(VideoPlayerEventType.Play, payload)

    expect(handler).not.toHaveBeenCalled()
  })

  it('should only unsubscribe the specified handler', () => {
    const handler1 = vi.fn()
    const handler2 = vi.fn()

    eventBus.on(VideoPlayerEventType.Play, handler1)
    eventBus.on(VideoPlayerEventType.Play, handler2)
    eventBus.off(VideoPlayerEventType.Play, handler1)

    const payload = { id: '123' }
    eventBus.emit(VideoPlayerEventType.Play, payload)

    expect(handler1).not.toHaveBeenCalled()
    expect(handler2).toHaveBeenCalledWith(payload)
  })

  it('should handle events with different payload types', () => {
    const handler = vi.fn()
    eventBus.on(VideoPlayerEventType.Play, handler)

    const payloads = [
      { id: '123' },
      { id: '123', duration: 120 },
      { error: new Error('Test error') },
      { metadata: { title: 'Test Video' } },
    ]

    payloads.forEach(payload => {
      eventBus.emit(VideoPlayerEventType.Play, payload)
      expect(handler).toHaveBeenCalledWith(payload)
    })
  })

  it('should not fail when emitting event with no handlers', () => {
    expect(() => {
      eventBus.emit(VideoPlayerEventType.Play, { id: '123' })
    }).not.toThrow()
  })

  it('should not fail when unsubscribing non-existent handler', () => {
    const handler = vi.fn()
    expect(() => {
      eventBus.off(VideoPlayerEventType.Play, handler)
    }).not.toThrow()
  })

  //   it('should maintain type safety for event types', () => {
  //     const handler: EventHandler = payload => {
  //       console.log(payload)
  //     }

  //     eventBus.on(VideoPlayerEventType.Play, handler)
  //   })
})
