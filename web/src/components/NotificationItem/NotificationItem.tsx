import { useEffect, useRef, useState } from 'react'

import {
  CheckBadgeIcon,
  FaceFrownIcon,
  XMarkIcon,
} from '@heroicons/react/24/outline'
import { IconButton, Typography } from '@material-tailwind/react'

import { DefaultNotificationTimeout, Variation } from './constants'
import { NotificationModel } from '~/contexts/NotificationContext'

export interface NotificationItemProps extends NotificationModel {
  onRemove: (id: string) => void
  variation?: Variation
}

export const NotificationItem = ({
  id,
  title,
  message,
  duration,
  onRemove,
  variation = Variation.Normal,
}: NotificationItemProps) => {
  const [isLeaving, setIsLeaving] = useState(false)
  const [isVisible, setIsVisible] = useState(false)
  const elementRef = useRef<HTMLDivElement>(null)
  const [height, setHeight] = useState(0)

  useEffect(() => {
    setIsVisible(true)
    setHeight(elementRef.current?.clientHeight || 0)

    const timer = setTimeout(() => {
      setIsLeaving(true)
      setIsVisible(false)

      // Remove the notification after the animation is complete
      setTimeout(() => {
        onRemove(id)
      }, 500)
    }, duration || DefaultNotificationTimeout)

    return () => clearTimeout(timer)
  }, [duration, id, message, onRemove])

  const handleRemove = () => {
    setIsVisible(false)
    setIsLeaving(true)

    setTimeout(() => {
      onRemove(id)
    }, 400)
  }

  console.log('variation', variation)

  const getAnimationClass = () => {
    switch (variation) {
      case Variation.Success:
        return 'transform transition-x duration-500 ease-[linear(0,_0.402_7.4%,_0.711_15.3%,_0.929_23.7%,_1.008_28.2%,_1.067_33%,_1.099_36.9%,_1.12_41%,_1.13_45.4%,_1.13_50.1%,_1.111_58.5%,_1.019_83.2%,_1.004_91.3%,_1)]'
      case Variation.Error:
        return 'transform transition-x duration-500 ease-[linear(0,_0.115_2.2%,_0.877_9.4%,_1.016_11.9%,_1.081_14.7%,_1.088_16%,_1.084_17.5%,_1.013_25.3%,_0.993_30.8%,_1.001_46.8%,_1)]'
      default:
        return 'bg-white'
    }
  }

  const getIcon = () => {
    switch (variation) {
      case Variation.Success:
        return <CheckBadgeIcon className='h-6 w-6 text-green-500 mr-2' />
      case Variation.Error:
        return <FaceFrownIcon className='h-6 w-6 text-red-500 mr-2' />
      default:
        return <CheckBadgeIcon className='h-6 w-6 text-green-500 mr-2' />
    }
  }

  return (
    <div
      ref={elementRef}
      className={`
        mb-2 p-4 rounded-lg shadow-lg
        flex flex-col items-start justify-between
        bg-white dark:bg-gray-800
        ${getAnimationClass()}
        ${isVisible ? 'translate-x-0 opacity-100' : 'translate-x-full opacity-0'}
        ${isLeaving ? 'pointer-events-none' : ''}
      `}
      style={{
        marginBottom: isLeaving ? `-${height}px` : '1rem',
      }}
    >
      <div className='flex w-full items-center justify-between mb-2'>
        {getIcon()}
        <Typography
          className='flex-grow text-lg font-medium'
          variant='paragraph'
        >
          {title}
        </Typography>
        <IconButton
          variant='text'
          size='sm'
          ripple={false}
          className='ml-auto text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'
          onClick={handleRemove}
        >
          <XMarkIcon className='h-5 w-5' />
        </IconButton>
      </div>
      <Typography
        className='ml-8 whitespace-pre-wrap break-words'
        variant='small'
      >
        {message}
      </Typography>
    </div>
  )
}
