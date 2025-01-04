import { FC } from 'react'
import { useTranslation } from 'react-i18next'

import { BaseDialog } from '../BaseDialog/BaseDialog'
import { LibraryDto } from '~/bindings/LibraryDto'
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
  const post = usePost()

  const onSubmit = async (data: LibraryDto) => {
    await post('/library/', data)
    dialogHandler()
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
