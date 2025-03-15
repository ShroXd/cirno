import { useEffect, useRef, useState } from 'react'
import { useTranslation } from 'react-i18next'

import { BadgeCheckIcon, FrownIcon, X } from 'lucide-react'

import { Button } from '../ui/button'
import { DefaultNotificationTimeout, Variation } from './constants'
import { NotificationModel } from '~/contexts/NotificationContext/NotificationContext.tsx'

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
  const [height, setHeight] = useState(0)

  const elementRef = useRef<HTMLDivElement>(null)
  const autoCloseTimer = useRef<NodeJS.Timeout | undefined>(undefined)

  const { t } = useTranslation()

  useEffect(() => {
    setIsVisible(true)
    setHeight(elementRef.current?.clientHeight || 0)

    autoCloseTimer.current = setTimeout(() => {
      setIsLeaving(true)
      setIsVisible(false)

      // Remove the notification after the animation is complete
      setTimeout(() => {
        onRemove(id)
      }, 500)
    }, duration || DefaultNotificationTimeout)

    return () => clearTimeout(autoCloseTimer.current)
  }, [duration, id, message, onRemove])

  const handleRemove = () => {
    setIsVisible(false)
    setIsLeaving(true)

    setTimeout(() => {
      clearTimeout(autoCloseTimer.current)
      onRemove(id)
    }, 400)
  }

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
        return (
          <BadgeCheckIcon
            data-testid='success-icon'
            className='mr-2 h-6 w-6 text-green-500'
          />
        )
      case Variation.Error:
        return (
          <FrownIcon
            data-testid='error-icon'
            className='mr-2 h-6 w-6 text-red-500'
          />
        )
      default:
        return (
          <BadgeCheckIcon
            data-testid='default-icon'
            className='mr-2 h-6 w-6 text-green-500'
          />
        )
    }
  }

  return (
    <div
      ref={elementRef}
      className={`mb-2 flex flex-col items-start justify-between rounded-lg bg-white p-4 shadow-lg dark:bg-gray-800 ${getAnimationClass()} ${isVisible ? 'translate-x-0 opacity-100' : 'translate-x-full opacity-0'} ${isLeaving ? 'pointer-events-none' : ''} `}
      style={{
        marginBottom: isLeaving ? `-${height}px` : '1rem',
      }}
    >
      <div className='mb-2 flex w-full items-center justify-between'>
        {getIcon()}
        <span className='flex-grow text-lg font-medium'>{title}</span>
        <Button
          aria-label={t('component.NotificationItem.close')}
          variant='outline'
          size='sm'
          className='ml-auto text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'
          onClick={handleRemove}
        >
          <X className='h-5 w-5' />
        </Button>
      </div>
      <span className='ml-8 whitespace-pre-wrap break-words'>{message}</span>
    </div>
  )
}
