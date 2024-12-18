import {
  Typography,
  List,
  ListItem,
  ListItemPrefix,
  Input,
  Accordion,
  AccordionBody,
  AccordionHeader,
  Button,
} from '@material-tailwind/react'
import {
  Cog6ToothIcon,
  LightBulbIcon,
  InboxStackIcon,
  HeartIcon,
  ChevronDownIcon,
  PlusIcon,
  TrashIcon,
} from '@heroicons/react/24/solid'
import { MagnifyingGlassIcon } from '@heroicons/react/24/outline'
import { useNavigate } from 'react-router-dom'
import { useTranslation } from 'react-i18next'
import { useState } from 'react'
import { mutate } from 'swr'

import { LibraryManageDialog } from '@/components/LibraryManageDialog/LibraryManageDialog'
import { useFetch } from '@/hooks/useFetch'
import { DeleteConfirmationDialog } from '@/components/DeleteConfirmationDialog/DeleteConfirmationDialog'
import { MediaLibraryDto } from '@/bindings/MediaLibraryDTO'

export const Sidebar = () => {
  const [expanded, setExpanded] = useState(0)
  const [mediaManageDialogOpen, setMediaManageDialogOpen] = useState(false)
  const [isManaging, setIsManaging] = useState(false)
  const [showDeleteConfirmation, setShowDeleteConfirmation] = useState(false)
  const [mediaLibraryToDelete, setMediaLibraryToDelete] = useState<
    number | null
  >(null)

  const { t } = useTranslation()
  const navigate = useNavigate()

  const { data, isLoading } = useFetch<MediaLibraryDto[]>('/media-libraries/')

  const toggleExpand = (value: number) => {
    setExpanded(expanded === value ? 0 : value)
    if (expanded !== 1) {
      setIsManaging(false)
    }
  }

  const toggleMediaManageDialog = () => {
    setMediaManageDialogOpen(!mediaManageDialogOpen)
  }

  const toggleIsManaging = () => {
    setIsManaging(!isManaging)
  }

  const handleDeleteMediaLibrary = (id: number) => {
    setMediaLibraryToDelete(id)
    setShowDeleteConfirmation(true)
  }

  const handleDeleteMediaLibraryConfirmation = () => {
    mutate(`/media-libraries/`)
    setShowDeleteConfirmation(false)
    setMediaLibraryToDelete(null)
  }

  // const { listenForMessages } = useEventBus()
  // listenForMessages('media_library_scanned', (payload: unknown) => {
  //   console.log('payload', payload)
  //   // TODO: update state of the specific media library
  //   setIsScanning(false)
  // })

  return (
    <>
      <LibraryManageDialog
        open={mediaManageDialogOpen}
        handleOpen={toggleMediaManageDialog}
      />
      <DeleteConfirmationDialog
        // TODO: handle guard value
        mediaLibraryId={mediaLibraryToDelete ?? 0}
        open={showDeleteConfirmation}
        handleOpen={handleDeleteMediaLibraryConfirmation}
        title={t('component.deleteConfirmationDialog.title')}
        description={t('component.deleteConfirmationDialog.description')}
      />
      <div className='w-full max-w-[20rem] p-4 border-r border-blue-gray-50 overflow-y-auto h-screen [&::-webkit-scrollbar]:hidden [-ms-overflow-style:none] [scrollbar-width:none]'>
        <div className='mb-2 flex items-center gap-4 p-4'>
          <img
            src='https://docs.material-tailwind.com/img/logo-ct-dark.png'
            alt='brand'
            className='h-8 w-8'
          />
          <Typography variant='h5' color='blue-gray'>
            Sidebar
          </Typography>
        </div>
        <div className='p-2'>
          <Input
            icon={<MagnifyingGlassIcon className='h-5 w-5' />}
            label='Search'
          />
        </div>
        <List>
          <Accordion
            open={expanded === 1}
            icon={
              <ChevronDownIcon
                strokeWidth={2.5}
                className={`mx-auto h-4 w-4 transition-transform ${
                  expanded === 1 ? 'rotate-180' : ''
                }`}
              />
            }
          >
            <div className='flex flex-row items-center gap-3'>
              <ListItem className='p-0' onClick={() => navigate('/')}>
                <AccordionHeader
                  onClick={() => toggleExpand(1)}
                  className='border-b-0 p-3'
                >
                  <ListItemPrefix>
                    <InboxStackIcon className='h-5 w-5' />
                  </ListItemPrefix>
                  <Typography color='blue-gray' className='mr-auto font-normal'>
                    {t('component.sidebar.library')}
                  </Typography>
                </AccordionHeader>
              </ListItem>
              {expanded === 1 ? (
                <Button
                  className='p-3 !overflow-visible'
                  variant='text'
                  ripple={false}
                  onClick={toggleIsManaging}
                >
                  <Cog6ToothIcon className='h-4 w-4' />
                </Button>
              ) : (
                <Button
                  className='p-3 !overflow-visible'
                  variant='gradient'
                  ripple={false}
                  onClick={toggleMediaManageDialog}
                >
                  <PlusIcon className='h-4 w-4' />
                </Button>
              )}
            </div>
            <AccordionBody className='py-1'>
              {!isLoading && (
                <List className='p-0'>
                  {data?.map(mediaLibrary => (
                    <div
                      className='flex flex-row justify-between gap-3'
                      key={mediaLibrary.id.toString()}
                    >
                      <ListItem className='pl-6'>{mediaLibrary.name}</ListItem>
                      <Button
                        className={`p-3 !overflow-visible transition-all duration-1000 ${isManaging ? 'opacity-100' : 'opacity-0 pointer-events-none hidden'}`}
                        color='red'
                        variant='text'
                        ripple={false}
                        onClick={() =>
                          handleDeleteMediaLibrary(Number(mediaLibrary.id))
                        }
                      >
                        <TrashIcon className='h-4 w-4' />
                      </Button>
                    </div>
                  ))}
                </List>
              )}
            </AccordionBody>
          </Accordion>
          <ListItem onClick={() => navigate('/favorites')}>
            <ListItemPrefix>
              <HeartIcon className='h-5 w-5' />
            </ListItemPrefix>
            <Typography color='blue-gray' className='mr-auto font-normal'>
              {t('component.sidebar.favorites')}
            </Typography>
          </ListItem>
          <hr className='my-2 border-blue-gray-50' />
          <ListItem onClick={() => navigate('/settings')}>
            <ListItemPrefix>
              <Cog6ToothIcon className='h-5 w-5' />
            </ListItemPrefix>
            {t('component.sidebar.settings')}
          </ListItem>
          <ListItem onClick={() => navigate('/test')}>
            <ListItemPrefix>
              <LightBulbIcon className='h-5 w-5' />
            </ListItemPrefix>
            {t('component.sidebar.test')}
          </ListItem>
        </List>
      </div>
    </>
  )
}
