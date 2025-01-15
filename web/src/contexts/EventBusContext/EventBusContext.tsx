import { FC, ReactNode, createContext, useCallback, useMemo } from 'react'

import { createEventBus } from './eventBus'
import { EventHandler, EventType, PayloadMap } from './types'

interface EventBusContextProps {
  emitEvent: <E extends EventType>(message: {
    event: E
    payload: PayloadMap[E]
  }) => void
  onEvent: <E extends EventType>(event: E, handler: EventHandler<E>) => void
  offEvent: <E extends EventType>(event: E, handler: EventHandler<E>) => void
}

export const EventBusContext = createContext<EventBusContextProps | undefined>(
  undefined
)

export const EventBusProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const eventBus = useMemo(() => createEventBus(), [])

  const emitEvent = useCallback<EventBusContextProps['emitEvent']>(
    message => {
      eventBus.emit(message.event, message.payload)
    },
    [eventBus]
  )

  const onEvent = useCallback<EventBusContextProps['onEvent']>(
    (event, handler) => {
      eventBus.on(event, handler)
    },
    [eventBus]
  )

  const offEvent = useCallback<EventBusContextProps['offEvent']>(
    (event, handler) => {
      eventBus.off(event, handler)
    },
    [eventBus]
  )

  const contextValue = useMemo(
    () => ({ emitEvent, onEvent, offEvent }),
    [emitEvent, onEvent, offEvent]
  )

  return (
    <EventBusContext.Provider value={contextValue}>
      {children}
    </EventBusContext.Provider>
  )
}
