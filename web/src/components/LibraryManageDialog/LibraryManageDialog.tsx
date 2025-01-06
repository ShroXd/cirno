import { FC, useCallback } from 'react'
import { Controller, useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'

import { FolderIcon, XMarkIcon } from '@heroicons/react/24/solid'
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

import { usePost } from '~/hooks/usePost'

interface LibraryManageDialogProps {
  open: boolean
  handleOpen: () => void
}

// TODO: add directory selection
export const LibraryManageDialog: FC<LibraryManageDialogProps> = ({
  open,
  handleOpen,
}) => {
  const { t } = useTranslation()
  const post = usePost()

  const {
    register,
    handleSubmit,
    control,
    reset,
    formState: { errors },
  } = useForm()

  const handleClose = useCallback(() => {
    reset()
    handleOpen()
  }, [reset, handleOpen])

  const onSubmit = handleSubmit(data => {
    post('/library/', data)
    handleClose()
  })

  return (
    <Dialog size='sm' open={open} handler={handleClose} className='p-4'>
      <form onSubmit={onSubmit}>
        <DialogHeader className='relative m-0 block'>
          <Typography variant='h4' color='blue-gray'>
            {t('component.libraryManageDialog.title')}
          </Typography>
          <Typography className='mt-1 font-normal text-gray-600'>
            {t('component.libraryManageDialog.description')}
          </Typography>
          <IconButton
            size='sm'
            variant='text'
            className='!absolute right-3.5 top-3.5'
            onClick={handleOpen}
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
              className='placeholder:opacity-100 focus:!border-t-gray-900'
              containerProps={{
                className: '!min-w-full',
              }}
              labelProps={{
                className: 'hidden',
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
                <Select
                  className='focus:!border-primary group-hover:!border-primary !w-full !border-[1.5px] !border-blue-gray-200/90 !border-t-blue-gray-200/90 bg-white text-gray-800 ring-4 ring-transparent placeholder:text-gray-600 focus:!border-blue-gray-900'
                  placeholder='1'
                  labelProps={{
                    className: 'hidden',
                  }}
                  {...field}
                >
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
                className='placeholder:opacity-100 focus:!border-t-gray-900'
                containerProps={{
                  className: '!min-w-full',
                }}
                labelProps={{
                  className: 'hidden',
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
          <Button className='ml-auto' type='submit'>
            {t('common.confirm')}
          </Button>
        </DialogFooter>
      </form>
    </Dialog>
  )
}
