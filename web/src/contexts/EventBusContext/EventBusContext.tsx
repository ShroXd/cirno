import { createContext, FC, ReactNode, useMemo } from 'react'
import { createEventBus, EventHandler, EventType } from './eventBus'

interface EventBusContextProps {
  emitEvent: (message: { event: EventType; payload: unknown }) => void
  onEvent: (event: EventType, handler: EventHandler) => void
}

export const EventBusContext = createContext<EventBusContextProps | undefined>(
  undefined
)

export const EventBusProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const eventBus = useMemo(() => createEventBus(), [])

  const emitEvent = (message: { event: EventType; payload: unknown }) => {
    eventBus.emit(message.event, message.payload)
  }

  const onEvent = (event: EventType, handler: EventHandler) => {
    eventBus.on(event, handler)
  }

  return (
    <EventBusContext.Provider value={{ emitEvent, onEvent }}>
      {children}
    </EventBusContext.Provider>
  )
}
