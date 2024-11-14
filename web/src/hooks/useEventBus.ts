import { useContext } from 'react'
import { EventBusContext } from '../contexts/EventBusContext/EventBusContext'

export const useEventBus = () => {
  const eventBus = useContext(EventBusContext)
  if (!eventBus) {
    throw new Error('useEventBus must be used within an EventBusProvider')
  }

  return eventBus
}
