import { FC, useState } from 'react'
import { useTranslation } from 'react-i18next'

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '~/components/ui/alert-dialog'

export interface ConfirmDialogProps {
  title: string
  description: string
  trigger: React.ReactNode
  onConfirm?: () => Promise<void>
  onCancel?: () => Promise<void>
}

const ConfirmDialog: FC<ConfirmDialogProps> = ({
  title,
  description,
  onConfirm,
  onCancel,
  trigger,
}) => {
  const [isOpen, setIsOpen] = useState(false)
  const { t } = useTranslation()

  const handleCancel = async () => {
    if (onCancel) {
      await onCancel()
    }
    setIsOpen(false)
  }

  const handleConfirm = async () => {
    if (onConfirm) {
      await onConfirm()
    }
    setIsOpen(false)
  }

  return (
    <AlertDialog open={isOpen} onOpenChange={setIsOpen}>
      <AlertDialogTrigger asChild>{trigger}</AlertDialogTrigger>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>{title}</AlertDialogTitle>
          <AlertDialogDescription>{description}</AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel onClick={handleCancel}>
            {t('common.cancel')}
          </AlertDialogCancel>
          <AlertDialogAction onClick={handleConfirm}>
            {t('common.confirm')}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}

export default ConfirmDialog
