import { FC, ReactNode, createContext, useCallback, useState } from 'react'

import { NotificationItem } from '~/components/NotificationItem/NotificationItem'
import { Variation } from '~/components/NotificationItem/constants'

export type NotificationModel = {
  id: string
  message: string
  title?: string
  type?: 'success' | 'error' | 'info' | 'warning' | (string & {})
  duration?: number
  onRemove?: (id: string) => void
  variation?: Variation
}

interface NotificationContextProps {
  addNotification: (
    notification: Omit<NotificationModel, 'id'>,
    variation?: Variation
  ) => string // id of the notification
  removeNotification: (id: string) => void
}

export const NotificationContext = createContext<NotificationContextProps>({
  addNotification: () => '',
  removeNotification: () => {},
})

export const NotificationProvider: FC<{ children: ReactNode }> = ({
  children,
}) => {
  const [notifications, setNotifications] = useState<NotificationModel[]>([])

  const addNotification = (
    notification: Omit<NotificationModel, 'id'>,
    variation?: Variation
  ) => {
    const id = Math.random().toString(36).substring(2, 15)
    setNotifications([...notifications, { id, ...notification, variation }])
    return id
  }

  const removeNotification = useCallback((id: string) => {
    setNotifications(prev => prev.filter(n => n.id !== id))
  }, [])

  // TODO: add error notification and animation
  // TODO: we should let addnotification accept a function to handle the error, like refresh the page

  return (
    <NotificationContext.Provider
      value={{ addNotification, removeNotification }}
    >
      {children}
      <div className='fixed right-0 top-0 z-[999999] w-full max-w-md p-4'>
        {notifications.map(n => (
          <NotificationItem
            {...n}
            key={n.id}
            onRemove={removeNotification}
            variation={n.variation}
          />
        ))}
      </div>
    </NotificationContext.Provider>
  )
}
