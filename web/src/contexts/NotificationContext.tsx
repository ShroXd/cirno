import { FC, ReactNode, createContext, useCallback, useState } from 'react'

import { NotificationItem } from '~/components/NotificationItem/NotificationItem'

export type NotificationModel = {
  id: string
  message: string
  title?: string
  type?: 'success' | 'error' | 'info' | 'warning' | (string & {})
  duration?: number
  onRemove?: (id: string) => void
}

interface NotificationContextProps {
  addNotification: (notification: Omit<NotificationModel, 'id'>) => string // id of the notification
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

  const addNotification = (notification: Omit<NotificationModel, 'id'>) => {
    const id = crypto.randomUUID()
    setNotifications([...notifications, { id, ...notification }])
    return id
  }

  const removeNotification = useCallback((id: string) => {
    setNotifications(prev => prev.filter(n => n.id !== id))
  }, [])

  return (
    <NotificationContext.Provider
      value={{ addNotification, removeNotification }}
    >
      {children}
      <div className='fixed top-0 right-0 w-full max-w-md p-4 z-50'>
        {notifications.map(n => (
          <NotificationItem {...n} key={n.id} onRemove={removeNotification} />
        ))}
      </div>
    </NotificationContext.Provider>
  )
}
