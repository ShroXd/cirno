import { FolderIcon, XMarkIcon } from '@heroicons/react/24/solid'
import {
  Dialog,
  DialogHeader,
  Typography,
  IconButton,
  DialogBody,
  Input,
  Select,
  Option,
  DialogFooter,
  Button,
} from '@material-tailwind/react'
import { FC } from 'react'
import { useTranslation } from 'react-i18next'

interface LibraryManageDialogProps {
  open: boolean
  handleOpen: () => void
}

export const LibraryManageDialog: FC<LibraryManageDialogProps> = ({
  open,
  handleOpen,
}) => {
  //   const [directoryHandle, setDirectoryHandle] =
  //     useState<FileSystemDirectoryHandle | null>(null)

  const { t } = useTranslation()

  // TODO: FileSystemDirectoryHandle can't return absoluted path
  //   const selectDirectory = async () => {
  //     try {
  //       // eslint-disable-next-line @typescript-eslint/no-explicit-any
  //       const handle = await (window as any).showDirectoryPicker()
  //       console.log(handle)
  //     } catch (error) {
  //       console.error('Error selecting directory:', JSON.stringify(error))
  //     }
  //   }

  return (
    <Dialog size='sm' open={open} handler={handleOpen} className='p-4'>
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
            name='name'
            className='placeholder:opacity-100 focus:!border-t-gray-900'
            containerProps={{
              className: '!min-w-full',
            }}
            labelProps={{
              className: 'hidden',
            }}
          />
        </div>
        <div>
          <Typography
            variant='small'
            color='blue-gray'
            className='mb-2 text-left font-medium'
          >
            {t('component.libraryManageDialog.category')}
          </Typography>
          <Select
            className='!w-full !border-[1.5px] !border-blue-gray-200/90 !border-t-blue-gray-200/90 bg-white text-gray-800 ring-4 ring-transparent placeholder:text-gray-600 focus:!border-primary focus:!border-blue-gray-900 group-hover:!border-primary'
            placeholder='1'
            labelProps={{
              className: 'hidden',
            }}
          >
            {/* TODO: get the options from the rs_ts generated enum */}
            <Option>
              {t('component.libraryManageDialog.categoryOptions.movie')}
            </Option>
            <Option>
              {t('component.libraryManageDialog.categoryOptions.tvSeries')}
            </Option>
            <Option>
              {t('component.libraryManageDialog.categoryOptions.animation')}
            </Option>
          </Select>
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
              name='size'
              className='placeholder:opacity-100 focus:!border-t-gray-900'
              containerProps={{
                className: '!min-w-full',
              }}
              labelProps={{
                className: 'hidden',
              }}
            />
            <div className='absolute right-1.5 top-1.5'>
              <IconButton size='sm' className='rounded' variant='text' disabled>
                <FolderIcon className='h-5 w-5' />
              </IconButton>
            </div>
          </div>
        </div>
      </DialogBody>
      <DialogFooter>
        <Button className='ml-auto' onClick={handleOpen}>
          {t('component.libraryManageDialog.confirm')}
        </Button>
      </DialogFooter>
    </Dialog>
  )
}
