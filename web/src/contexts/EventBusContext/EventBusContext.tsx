import { createContext, FC, ReactNode, useMemo } from 'react'
import { createEventBus, EventHandler, AllEvents } from './eventBus'

interface EventBusContextProps {
  sendMessage: (message: { event: AllEvents; payload: unknown }) => void
  listenForMessages: (event: AllEvents, handler: EventHandler) => void
}

export const EventBusContext = createContext<EventBusContextProps | undefined>(
  undefined
)

export const EventBusProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const eventBus = useMemo(() => createEventBus(), [])

  const sendMessage = (message: { event: AllEvents; payload: unknown }) => {
    eventBus.emit(message.event, message.payload)
  }

  const listenForMessages = (event: AllEvents, handler: EventHandler) => {
    eventBus.on(event, handler)
  }

  return (
    <EventBusContext.Provider value={{ sendMessage, listenForMessages }}>
      {children}
    </EventBusContext.Provider>
  )
}
