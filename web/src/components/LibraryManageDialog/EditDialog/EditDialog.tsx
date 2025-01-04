import { FC } from 'react'
import { useTranslation } from 'react-i18next'

import { BaseDialog } from '../BaseDialog/BaseDialog'
import { LibraryDto } from '~/bindings/LibraryDto'
import { useFetch } from '~/hooks/useFetch'
import { usePost } from '~/hooks/usePost'

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
  // TODO: create a global error handler and notification
  const { data, isLoading, error } = useFetch<LibraryDto>(
    open ? `/library/${libraryId}` : null
  )
  const post = usePost()

  const onSubmit = (data: LibraryDto) => {
    post('/library/', data)
    dialogHandler()
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
