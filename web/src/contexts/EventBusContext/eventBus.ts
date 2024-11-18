import { EventName } from '../../bindings/EventName'

export type EventHandler = (payload: unknown) => void

// TODO: 1. async tasks

export type AllEvents = EventName

export type EventBusType = {
  on: (event: AllEvents, handler: EventHandler) => void
  off: (event: AllEvents, handler: EventHandler) => void
  emit: (event: AllEvents, payload: unknown) => void
}

/**
 * Creates an event bus instance for handling pub/sub events.
 * Returns an object with methods for subscribing to and emitting events.
 * @returns {Object} Event bus methods {on, off, emit}
 */
export const createEventBus = (): EventBusType => {
  const registry = new Map<AllEvents, EventHandler[]>()

  const on = (event: AllEvents, handler: EventHandler) => {
    const eventHandlers = registry.get(event) || []
    eventHandlers.push(handler)
    registry.set(event, eventHandlers)
  }

  const off = (event: AllEvents, handler: EventHandler) => {
    const eventHandlers = registry.get(event) || []
    registry.set(
      event,
      eventHandlers.filter(h => h !== handler)
    )
  }

  const emit = (event: AllEvents, payload: unknown) => {
    const eventHandlers = registry.get(event) || []
    eventHandlers.forEach(handler => handler(payload))
  }

  return { on, off, emit }
}
