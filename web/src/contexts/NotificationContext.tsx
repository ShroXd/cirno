import { createContext, FC, ReactNode, useState } from 'react'
import { NotificationItem } from '../components/NotificationItem/NotificationItem'

export type NotificationModel = {
  id: string
  message: string
  title?: string
  type?: 'success' | 'error' | 'info' | 'warning' | (string & {})
  duration?: number
  onRemove?: () => void
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

  const removeNotification = (id: string) => {
    setNotifications(prev => prev.filter(n => n.id !== id))
  }

  return (
    <NotificationContext.Provider
      value={{ addNotification, removeNotification }}
    >
      {children}

      <div className='fixed bottom-0 right-0 w-full max-w-md p-4'>
        {notifications.map(n => (
          <NotificationItem
            key={n.id}
            notification={n}
            onRemove={() => removeNotification(n.id)}
          />
        ))}
      </div>
    </NotificationContext.Provider>
  )
}
