import { createContext, FC, ReactNode, useMemo } from 'react'
import { createEventBus, EventHandler } from './eventBus'

interface EventBusContextProps {
  sendMessage: (message: { event: string; payload: unknown }) => void
  listenForMessages: (event: string, handler: EventHandler) => void
}

export const EventBusContext = createContext<EventBusContextProps | undefined>(
  undefined
)

export const EventBusProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const eventBus = useMemo(() => createEventBus(), [])

  const sendMessage = (message: { event: string; payload: unknown }) => {
    eventBus.emit(message.event, message.payload)
  }

  const listenForMessages = (event: string, handler: EventHandler) => {
    eventBus.on(event, handler)
  }

  return (
    <EventBusContext.Provider value={{ sendMessage, listenForMessages }}>
      {children}
    </EventBusContext.Provider>
  )
}
