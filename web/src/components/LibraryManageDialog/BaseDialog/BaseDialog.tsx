import { FC, useCallback, useEffect } from 'react'
import { Controller, useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'

import { FolderIcon, XMarkIcon } from '@heroicons/react/24/outline'
import {
  Button,
  Dialog,
  DialogBody,
  DialogFooter,
  DialogHeader,
  IconButton,
  Input,
  Option,
  Select,
  Typography,
} from '@material-tailwind/react'

import { LibraryDto } from '~/bindings/LibraryDto'

interface BaseDialogProps {
  title: string
  description: string
  submitButtonText: string
  open: boolean
  defaultValues?: LibraryDto

  onSubmit: (data: LibraryDto) => Promise<void>
  dialogHandler: () => void
  onClose?: () => void
}

export const BaseDialog: FC<BaseDialogProps> = ({
  title,
  description,
  submitButtonText,
  open,
  defaultValues,
  onSubmit,
  dialogHandler,
  onClose,
}) => {
  const {
    register,
    handleSubmit,
    control,
    formState: { errors },
    reset,
  } = useForm<LibraryDto>({ values: defaultValues, defaultValues })
  const { t } = useTranslation()

  const handleDialogClose = useCallback(() => {
    dialogHandler()
    onClose?.()
  }, [dialogHandler, onClose])

  useEffect(() => {
    if (open) {
      reset(defaultValues)
    }
  }, [open, reset, defaultValues])

  return (
    <Dialog
      size='sm'
      open={open}
      handler={handleDialogClose}
      dismiss={{
        outsidePress: false,
      }}
      className='p-4'
      data-testid='library-manage-dialog'
    >
      <form onSubmit={handleSubmit(onSubmit)}>
        <DialogHeader className='relative m-0 block'>
          <Typography variant='h4' color='blue-gray' data-testid='dialog-title'>
            {title}
          </Typography>
          <Typography className='mt-1 font-normal text-gray-600'>
            {description}
          </Typography>
          <IconButton
            size='sm'
            variant='text'
            className='!absolute right-3.5 top-3.5'
            onClick={handleDialogClose}
          >
            <XMarkIcon className='h-4 w-4 stroke-2' />
          </IconButton>
        </DialogHeader>

        <DialogBody className='space-y-4 pb-6'>
          <div>
            <Typography
              variant='small'
              color='blue-gray'
              className='mb-2 text-left font-medium'
            >
              {t('component.libraryManageDialog.name')}
            </Typography>
            <Input
              color='gray'
              size='lg'
              placeholder={t('component.libraryManageDialog.namePlaceholder')}
              {...register('name', { required: true })}
              className='!border-t-blue-gray-200 placeholder:opacity-100 focus:!border-t-gray-900'
              containerProps={{
                className: '!min-w-full',
              }}
              labelProps={{
                className: 'before:content-none after:content-none',
              }}
            />
            {errors.name && (
              <Typography variant='small' color='red' className='mt-1'>
                {t('component.libraryManageDialog.nameRequired')}
              </Typography>
            )}
          </div>
          <div>
            <Typography
              variant='small'
              color='blue-gray'
              className='mb-2 text-left font-medium'
            >
              {t('component.libraryManageDialog.category')}
            </Typography>
            <Controller
              name='category'
              defaultValue='Movie'
              control={control}
              rules={{ required: true }}
              render={({ field }) => (
                <Select placeholder='1' {...field}>
                  {/* TODO: get the options & its values from the rs_ts generated enum */}
                  <Option value='Movie'>
                    {t('component.libraryManageDialog.categoryOptions.movie')}
                  </Option>
                  <Option value='TvShow'>
                    {t('component.libraryManageDialog.categoryOptions.tvShow')}
                  </Option>
                  <Option value='Animation'>
                    {t(
                      'component.libraryManageDialog.categoryOptions.animation'
                    )}
                  </Option>
                </Select>
              )}
            ></Controller>
            {errors.category && (
              <Typography variant='small' color='red' className='mt-1'>
                {t('component.libraryManageDialog.categoryRequired')}
              </Typography>
            )}
          </div>
          <div className='flex flex-col gap-4'>
            <Typography
              variant='small'
              color='blue-gray'
              className='text-left font-medium'
            >
              {t('component.libraryManageDialog.directory')}
            </Typography>
            <div className='relative -mt-2 w-full'>
              <Input
                color='gray'
                size='lg'
                placeholder={t(
                  'component.libraryManageDialog.directoryPlaceholder'
                )}
                {...register('directory', { required: true })}
                className='!border-t-blue-gray-200 placeholder:opacity-100 focus:!border-t-gray-900'
                containerProps={{
                  className: '!min-w-full',
                }}
                labelProps={{
                  className: 'before:content-none after:content-none',
                }}
              />
              <div className='absolute right-1.5 top-1.5'>
                <IconButton
                  size='sm'
                  className='rounded'
                  variant='text'
                  disabled
                >
                  <FolderIcon className='h-5 w-5' />
                </IconButton>
              </div>
            </div>
            {errors.directory && (
              <Typography variant='small' color='red' className='-mt-3'>
                {t('component.libraryManageDialog.directoryRequired')}
              </Typography>
            )}
          </div>
        </DialogBody>
        <DialogFooter>
          <Button className='ml-auto' type='submit' data-testid='submit-button'>
            {submitButtonText}
          </Button>
        </DialogFooter>
      </form>
    </Dialog>
  )
}
