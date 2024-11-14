import { createContext, FC, ReactNode, useMemo } from 'react'
import { createEventBus } from './eventBus'
import { EventBusType } from './eventBus'

export const EventBusContext = createContext<EventBusType | undefined>(
  undefined
)

export const EventBusProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const eventBus = useMemo(() => createEventBus(), [])

  return (
    <EventBusContext.Provider value={eventBus}>
      {children}
    </EventBusContext.Provider>
  )
}
