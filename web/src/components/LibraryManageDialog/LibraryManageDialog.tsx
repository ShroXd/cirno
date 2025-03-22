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
  const { emitEvent } = useEventBus()

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
    { value: 'Movie', label: '电影', icon: MonitorPlay },
    { value: 'TvShow', label: '电视剧', icon: FileVideo },
    { value: 'Animation', label: '动画', icon: FileVideo },
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
      mutate('/library/')
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
            <DialogTitle>添加媒体库</DialogTitle>
            <DialogDescription>
              添加新的媒体库路径到您的系统中
            </DialogDescription>
          </DialogHeader>
          <div className='grid gap-4 py-4'>
            <div className='grid gap-2'>
              <Label htmlFor='name'>媒体库名称</Label>
              <Input
                id='name'
                placeholder='例如：电影库、电视剧库'
                {...register('name', { required: true })}
              />
              {errors.name && (
                <span className='mt-1 text-xs text-red-500'>
                  {t('component.libraryManageDialog.nameRequired')}
                </span>
              )}
            </div>
            <div className='grid gap-2'>
              <Label htmlFor='path'>媒体库路径</Label>
              <div className='flex gap-2'>
                <Input
                  id='path'
                  placeholder='/media/path'
                  {...register('directory', { required: true })}
                  className='flex-1'
                />
                <Button variant='outline' onClick={handleBrowsePath}>
                  浏览...
                </Button>
              </div>
              {errors.directory && (
                <span className='mt-1 text-xs text-red-500'>
                  {t('component.libraryManageDialog.directoryRequired')}
                </span>
              )}
              <p className='text-xs text-muted-foreground'>
                选择包含媒体文件的文件夹路径
              </p>
            </div>
            <div className='grid grid-cols-1 gap-4 md:grid-cols-2'>
              <div className='grid gap-2'>
                <Label htmlFor='type'>媒体库类型</Label>
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
                        <SelectValue placeholder='选择媒体库类型' />
                      </SelectTrigger>
                      <SelectContent>
                        {libraryTypes.map(type => (
                          <SelectItem key={type.value} value={type.value}>
                            <div className='flex items-center gap-2'>
                              <type.icon className='h-4 w-4' />
                              <span>{type.label}</span>
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
                <Label htmlFor='auto-scan-new'>启用自动扫描</Label>
              </div>
            </div>
          </div>
          <DialogFooter>
            <DialogClose asChild>
              <Button variant='outline'>取消</Button>
            </DialogClose>
            <Button type='submit'>添加媒体库</Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
