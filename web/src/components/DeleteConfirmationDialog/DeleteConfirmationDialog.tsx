import { useTranslation } from 'react-i18next'

import {
  Button,
  Dialog,
  DialogBody,
  DialogFooter,
  DialogHeader,
} from '@material-tailwind/react'

interface DeleteConfirmationDialogProps {
  title: string
  description: string
  open: boolean
  handleConfirm: () => void
  handleCancel: () => void
}

export const DeleteConfirmationDialog = ({
  title,
  description,
  open,
  handleConfirm,
  handleCancel,
}: DeleteConfirmationDialogProps) => {
  const { t } = useTranslation()

  return (
    <Dialog open={open} handler={handleCancel} size='xs'>
      <DialogHeader>{title}</DialogHeader>
      <DialogBody>{description}</DialogBody>
      <DialogFooter className='flex justify-end gap-4'>
        <Button variant='text' onClick={handleCancel}>
          {t('common.cancel')}
        </Button>
        <Button color='red' onClick={handleConfirm}>
          {t('common.confirm')}
        </Button>
      </DialogFooter>
    </Dialog>
  )
}
