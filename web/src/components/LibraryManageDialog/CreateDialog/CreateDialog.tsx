import { FC } from 'react'
import { useTranslation } from 'react-i18next'

import { BaseDialog } from '../BaseDialog/BaseDialog'
import { AsyncTaskResponse } from '~/bindings/AsyncTaskResponse'
import { LibraryDto } from '~/bindings/LibraryDto'
import { useEventBus } from '~/hooks/useEventBus'
import { usePost } from '~/hooks/usePost'

interface CreateDialogProps {
  open: boolean
  dialogHandler: () => void
  onClose?: () => void
}

export const CreateDialog: FC<CreateDialogProps> = ({
  open,
  dialogHandler,
  onClose,
}) => {
  const post = usePost()
  const { t } = useTranslation()
  const { emitEvent } = useEventBus()

  const onSubmit = async (data: LibraryDto) => {
    try {
      const response = await post<LibraryDto, AsyncTaskResponse<bigint>>(
        '/library/',
        data
      )

      console.log('create dialog response', response.payload)
      emitEvent({
        event: 'LibraryScanning',
        payload: {
          libraryId: Number(response.payload),
        },
      })

      dialogHandler()
    } catch (error) {
      console.error('Failed to create media library', error)

      emitEvent({
        event: 'Error',
        payload: {
          title: t('component.libraryManageDialog.create.error.title'),
          message: t('component.libraryManageDialog.create.error.message'),
        },
      })
    }
  }

  return (
    <BaseDialog
      title={t('component.libraryManageDialog.create.title')}
      description={t('component.libraryManageDialog.create.description')}
      submitButtonText={t(
        'component.libraryManageDialog.create.submitButtonText'
      )}
      open={open}
      dialogHandler={dialogHandler}
      onSubmit={onSubmit}
      onClose={onClose}
    />
  )
}
