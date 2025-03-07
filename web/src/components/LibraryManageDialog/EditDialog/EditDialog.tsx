import { FC } from 'react'
import { useTranslation } from 'react-i18next'

import { mutate } from 'swr'

import { BaseDialog } from '../BaseDialog/BaseDialog'
import { LibraryDto } from '~/bindings/LibraryDto'
import { UpdateLibraryPayload } from '~/bindings/UpdateLibraryPayload'
import { useEventBus } from '~/hooks/useEventBus'
import { useFetch } from '~/hooks/useFetch'
import { usePut } from '~/hooks/usePut'

interface EditDialogProps {
  libraryId: number
  open: boolean
  dialogHandler: () => void
  onClose?: () => void
}

export const EditDialog: FC<EditDialogProps> = ({
  libraryId,
  open,
  dialogHandler,
  onClose,
}) => {
  const { t } = useTranslation()
  const { emitEvent } = useEventBus()
  // TODO: create a global error handler and notification
  const { data } = useFetch<LibraryDto>(open ? `/library/${libraryId}` : null)
  const put = usePut()

  const onSubmit = async (data: LibraryDto) => {
    const updatePayload: UpdateLibraryPayload = {
      id: libraryId,
      name: data.name,
      directory: data.directory,
      category: data.category,
    }

    try {
      await put(`/library/${libraryId}`, updatePayload)
      mutate(`/library/${libraryId}`)
      dialogHandler()
    } catch (error) {
      console.error('Failed to update media library', error)

      emitEvent({
        event: 'Error',
        payload: {
          title: t('component.libraryManageDialog.edit.error.title'),
          message: t('component.libraryManageDialog.edit.error.message'),
        },
      })
    }
  }

  return (
    <BaseDialog
      title={t('component.libraryManageDialog.edit.title')}
      description={t('component.libraryManageDialog.edit.description')}
      submitButtonText={t(
        'component.libraryManageDialog.edit.submitButtonText'
      )}
      defaultValues={data}
      open={open}
      onClose={onClose}
      onSubmit={onSubmit}
      dialogHandler={dialogHandler}
    />
  )
}
