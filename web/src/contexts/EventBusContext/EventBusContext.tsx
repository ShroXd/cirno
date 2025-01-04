import { FC, ReactNode, createContext, useMemo } from 'react'

import { createEventBus } from './eventBus'
import { EventHandler, EventType, PayloadMap } from './types'

interface EventBusContextProps {
  emitEvent: <E extends EventType>(message: {
    event: E
    payload: PayloadMap[E]
  }) => void
  onEvent: <E extends EventType>(event: E, handler: EventHandler<E>) => void
}

export const EventBusContext = createContext<EventBusContextProps | undefined>(
  undefined
)

export const EventBusProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const eventBus = useMemo(() => createEventBus(), [])

  const emitEvent = <E extends EventType>(message: {
    event: E
    payload: PayloadMap[E]
  }) => {
    eventBus.emit(message.event, message.payload)
  }

  const onEvent = <E extends EventType>(event: E, handler: EventHandler<E>) => {
    eventBus.on(event, handler)
  }

  return (
    <EventBusContext.Provider value={{ emitEvent, onEvent }}>
      {children}
    </EventBusContext.Provider>
  )
}
