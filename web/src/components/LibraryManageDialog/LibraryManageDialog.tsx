import { Dispatch, FC, SetStateAction, useEffect, useState } from 'react'
import { Controller, useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'

import { FileVideo, MonitorPlay } from 'lucide-react'
import { mutate } from 'swr'

import { AsyncTaskResponse } from '~/bindings/AsyncTaskResponse'
import { LibraryDto } from '~/bindings/LibraryDto'
import { Button } from '~/components/ui/button'
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '~/components/ui/dialog'
import { Input } from '~/components/ui/input'
import { Label } from '~/components/ui/label'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '~/components/ui/select'
import { Switch } from '~/components/ui/switch'
import { useEventBus } from '~/hooks/useEventBus'
import { usePost } from '~/hooks/usePost'

interface LibraryManageDialogProps {
  open: boolean
  defaultValues?: LibraryDto
  dialogHandler: Dispatch<SetStateAction<boolean>>
  onClose?: () => void
}

export const LibraryManageDialog: FC<LibraryManageDialogProps> = ({
  open,
  defaultValues,
  dialogHandler,
}) => {
  const [formData, setFormData] = useState({
    name: '',
    path: '',
    type: 'movies',
    autoScan: true,
  })

  const post = usePost()
  const { t } = useTranslation()
  const { emitEvent, onEvent } = useEventBus()

  const {
    register,
    handleSubmit,
    control,
    formState: { errors },
    reset,
  } = useForm<LibraryDto>({
    values: defaultValues,
    defaultValues,
  })

  useEffect(() => {
    if (open) {
      reset(defaultValues)
    }
  }, [open, reset, defaultValues])

  const libraryTypes = [
    {
      value: 'Movie',
      label: 'component.libraryManageDialog.form.type.options.movie',
      icon: MonitorPlay,
    },
    {
      value: 'TvShow',
      label: 'component.libraryManageDialog.form.type.options.tv',
      icon: FileVideo,
    },
    {
      value: 'Animation',
      label: 'component.libraryManageDialog.form.type.options.anime',
      icon: FileVideo,
    },
  ]

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

      dialogHandler(false)
      onEvent('LibrarySaved', () => {
        mutate('/library/')
      })
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

  const handleBrowsePath = () => {
    // In a real app, this would open a file browser dialog
    // For this demo, we'll just simulate it
    setTimeout(() => {
      setFormData({
        ...formData,
        path: '/selected/media/path',
      })
    }, 500)
  }

  return (
    <Dialog open={open} onOpenChange={dialogHandler}>
      <DialogContent className='sm:max-w-[550px]'>
        <form onSubmit={handleSubmit(onSubmit)}>
          <DialogHeader>
            <DialogTitle>
              {t('component.libraryManageDialog.add.title')}
            </DialogTitle>
            <DialogDescription>
              {t('component.libraryManageDialog.add.description')}
            </DialogDescription>
          </DialogHeader>
          <div className='grid gap-4 py-4'>
            <div className='grid gap-2'>
              <Label htmlFor='name'>
                {t('component.libraryManageDialog.form.name.label')}
              </Label>
              <Input
                id='name'
                placeholder={t(
                  'component.libraryManageDialog.form.name.placeholder'
                )}
                {...register('name', { required: true })}
              />
              {errors.name && (
                <span className='mt-1 text-xs text-red-500'>
                  {t(
                    'component.libraryManageDialog.form.name.rule.nameRequired'
                  )}
                </span>
              )}
            </div>
            <div className='grid gap-2'>
              <Label htmlFor='path'>
                {t('component.libraryManageDialog.form.path.label')}
              </Label>
              <div className='flex gap-2'>
                <Input
                  id='path'
                  placeholder={t(
                    'component.libraryManageDialog.form.path.placeholder'
                  )}
                  {...register('directory', { required: true })}
                  className='flex-1'
                />
                <Button variant='outline' onClick={handleBrowsePath}>
                  {t('component.libraryManageDialog.form.path.action.browse')}
                </Button>
              </div>
              {errors.directory ? (
                <span className='mt-1 text-xs text-red-500'>
                  {t(
                    'component.libraryManageDialog.form.path.rule.pathRequired'
                  )}
                </span>
              ) : (
                <p className='text-xs text-muted-foreground'>
                  {t('component.libraryManageDialog.form.path.hint')}
                </p>
              )}
            </div>
            <div className='grid grid-cols-1 gap-4 md:grid-cols-2'>
              <div className='grid gap-2'>
                <Label htmlFor='type'>
                  {t('component.libraryManageDialog.form.type.label')}
                </Label>
                <Controller
                  name='category'
                  defaultValue='Movie'
                  control={control}
                  rules={{ required: true }}
                  render={({
                    field: { onChange, value, name, ref, ...rest },
                  }) => (
                    <Select
                      onValueChange={onChange}
                      value={value}
                      name={name}
                      {...rest}
                    >
                      <SelectTrigger>
                        <SelectValue
                          placeholder={t(
                            'component.libraryManageDialog.form.type.placeholder'
                          )}
                        />
                      </SelectTrigger>
                      <SelectContent>
                        {libraryTypes.map(type => (
                          <SelectItem key={type.value} value={type.value}>
                            <div className='flex items-center gap-2'>
                              <type.icon className='h-4 w-4' />
                              <span>{t(type.label)}</span>
                            </div>
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  )}
                />
              </div>
              <div className='flex items-center space-x-2 self-end'>
                <Switch id='auto-scan-new' />
                <Label htmlFor='auto-scan-new'>
                  {t('component.libraryManageDialog.form.autoScan.label')}
                </Label>
              </div>
            </div>
          </div>
          <DialogFooter>
            <DialogClose asChild>
              <Button variant='outline'>
                {t('component.libraryManageDialog.action.cancel')}
              </Button>
            </DialogClose>
            <Button type='submit'>
              {t('component.libraryManageDialog.action.add')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
