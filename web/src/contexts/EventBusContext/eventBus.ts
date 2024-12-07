import { NotificationType } from '../../bindings/NotificationType'

export type EventHandler = (payload: unknown) => void

// TODO: 1. async tasks

export type EventType = NotificationType

export type EventBusType = {
  on: (event: EventType, handler: EventHandler) => void
  off: (event: EventType, handler: EventHandler) => void
  emit: (event: EventType, payload: unknown) => void
}

/**
 * Creates an event bus instance for handling pub/sub events.
 * Returns an object with methods for subscribing to and emitting events.
 * @returns {Object} Event bus methods {on, off, emit}
 */
export const createEventBus = (): EventBusType => {
  const registry = new Map<EventType, EventHandler[]>()

  const on = (event: EventType, handler: EventHandler) => {
    const eventHandlers = registry.get(event) || []
    eventHandlers.push(handler)
    registry.set(event, eventHandlers)
  }

  const off = (event: EventType, handler: EventHandler) => {
    const eventHandlers = registry.get(event) || []
    registry.set(
      event,
      eventHandlers.filter(h => h !== handler)
    )
  }

  const emit = (event: EventType, payload: unknown) => {
    const eventHandlers = registry.get(event) || []
    eventHandlers.forEach(handler => handler(payload))
  }

  return { on, off, emit }
}
