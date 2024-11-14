type EventHandler = (payload: unknown) => void

export type EventBusType = {
  on: (event: string, handler: EventHandler) => void
  off: (event: string, handler: EventHandler) => void
  emit: (event: string, payload: unknown) => void
}

/**
 * Creates an event bus instance for handling pub/sub events.
 * Returns an object with methods for subscribing to and emitting events.
 * @returns {Object} Event bus methods {on, off, emit}
 */
export const createEventBus = (): EventBusType => {
  const registry = new Map<string, EventHandler[]>()

  const on = (event: string, handler: EventHandler) => {
    const eventHandlers = registry.get(event) || []
    eventHandlers.push(handler)
    registry.set(event, eventHandlers)
  }

  const off = (event: string, handler: EventHandler) => {
    const eventHandlers = registry.get(event) || []
    registry.set(
      event,
      eventHandlers.filter(h => h !== handler)
    )
  }

  const emit = (event: string, payload: unknown) => {
    const eventHandlers = registry.get(event) || []
    eventHandlers.forEach(handler => handler(payload))
  }

  return { on, off, emit }
}
