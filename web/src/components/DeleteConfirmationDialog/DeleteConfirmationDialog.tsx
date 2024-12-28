import { useTranslation } from 'react-i18next'

import {
  Button,
  Dialog,
  DialogBody,
  DialogFooter,
  DialogHeader,
} from '@material-tailwind/react'

import { useDelete } from '~/hooks/useDelete'

interface DeleteConfirmationDialogProps {
  mediaLibraryId: number
  title: string
  description: string
  open: boolean
  handleOpen: () => void
}

export const DeleteConfirmationDialog = ({
  mediaLibraryId,
  title,
  description,
  open,
  handleOpen,
}: DeleteConfirmationDialogProps) => {
  const { t } = useTranslation()
  const del = useDelete()

  const handleDelete = async () => {
    console.log('delete media library', mediaLibraryId)
    const res = await del(`/library/${mediaLibraryId}`)
    console.log('res', res)
    handleOpen()
  }

  return (
    <Dialog open={open} handler={handleOpen} size='xs'>
      <DialogHeader>{title}</DialogHeader>
      <DialogBody>{description}</DialogBody>
      <DialogFooter className='flex justify-end gap-4'>
        <Button variant='text' onClick={handleOpen}>
          {t('common.cancel')}
        </Button>
        <Button color='red' onClick={handleDelete}>
          {t('common.confirm')}
        </Button>
      </DialogFooter>
    </Dialog>
  )
}
