import { FC } from 'react'
import { useTranslation } from 'react-i18next'

import { BaseDialog } from '../BaseDialog/BaseDialog'
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
  const { t } = useTranslation()
  const { emitEvent } = useEventBus()
  const post = usePost()

  const onSubmit = async (data: LibraryDto) => {
    try {
      await post('/library/', data)
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
