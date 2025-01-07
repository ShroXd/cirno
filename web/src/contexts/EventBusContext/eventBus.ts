import { EventBusType, EventHandler, EventType, PayloadMap } from './types'

// TODO: 1. async tasks

/**
 * Creates an event bus instance for handling pub/sub events.
 * Returns an object with methods for subscribing to and emitting events.
 * @returns {Object} Event bus methods {on, off, emit}
 */
export const createEventBus = (): EventBusType => {
  // eslint-disable-next-line @typescript-eslint/no-unsafe-function-type
  const registry = new Map<EventType, Function[]>()

  const on = <E extends EventType>(event: E, handler: EventHandler<E>) => {
    const eventHandlers = registry.get(event) || []
    // eslint-disable-next-line @typescript-eslint/no-unsafe-function-type
    eventHandlers.push(handler as Function)
    registry.set(event, eventHandlers)
  }

  const off = <E extends EventType>(event: E, handler: EventHandler<E>) => {
    const eventHandlers = registry.get(event) || []
    registry.set(
      event,
      eventHandlers.filter(h => h !== handler)
    )
  }

  const emit = <E extends EventType>(event: E, payload: PayloadMap[E]) => {
    const eventHandlers = registry.get(event) || []
    console.log('handlers', eventHandlers)
    eventHandlers.forEach(handler => (handler as EventHandler<E>)(payload))
  }

  return { on, off, emit }
}
