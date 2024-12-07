import { XMarkIcon } from '@heroicons/react/16/solid'
import { IconButton } from '@material-tailwind/react'
import { useEffect } from 'react'
import { NotificationType } from '../../contexts/NotificationContext'

export type NotificationItemProps = {
  notification: NotificationType
  onRemove: (id: string) => void
}

export const DefaultNotificationTimeout = 5000

export const NotificationItem = ({
  notification,
  onRemove,
}: NotificationItemProps) => {
  useEffect(() => {
    setTimeout(() => {
      onRemove(notification.id)
      notification.onRemove?.()
    }, notification.duration || DefaultNotificationTimeout)
  }, [notification, onRemove])

  const handleRemove = () => {
    onRemove(notification.id)
    notification.onRemove?.()
  }

  return (
    <div
      className={`
      mb-2 p-4 rounded-lg shadow-lg
      flex items-center justify-between
      bg-white dark:bg-gray-800
      transform transition-all duration-500 ease-in-out
      animate-slide-in
    `}
    >
      <span className='mr-8'>{notification.message}</span>
      <IconButton
        variant='text'
        className='text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'
        onClick={handleRemove}
      >
        <XMarkIcon className='h-5 w-5' />
      </IconButton>
    </div>
  )
}
