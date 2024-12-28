import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { useNavigate } from 'react-router-dom'

import { ChevronDownIcon } from '@heroicons/react/24/outline'
import {
  Accordion,
  AccordionBody,
  AccordionHeader,
  Card,
  Drawer,
  List,
  ListItem,
  ListItemPrefix,
  Typography,
} from '@material-tailwind/react'
import { mutate } from 'swr'

import { getIconAccordingToCategory } from './utils'
import { LibraryDto } from '~/bindings/LibraryDto'
import { DeleteConfirmationDialog } from '~/components/DeleteConfirmationDialog/DeleteConfirmationDialog'
import { useFetch } from '~/hooks/useFetch'

interface SidebarProps {
  open: boolean
  onClose: () => void
}

export const Sidebar = ({ open, onClose }: SidebarProps) => {
  const [expanded, setExpanded] = useState(true)
  const [showDeleteConfirmation, setShowDeleteConfirmation] = useState(false)
  const [mediaLibraryToDelete, setMediaLibraryToDelete] = useState<
    number | null
  >(null)

  const { t } = useTranslation()
  const navigate = useNavigate()

  const { data, isLoading } = useFetch<LibraryDto[]>('/library/')

  const toggleExpand = () => {
    setExpanded(!expanded)
  }

  const handleDeleteMediaLibraryConfirmation = () => {
    mutate(`/library/`)
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
      <DeleteConfirmationDialog
        mediaLibraryId={mediaLibraryToDelete ?? 0}
        open={showDeleteConfirmation}
        handleOpen={handleDeleteMediaLibraryConfirmation}
        title={t('component.deleteConfirmationDialog.title')}
        description={t('component.deleteConfirmationDialog.description')}
      />
      <Drawer open={open} onClose={onClose}>
        <Card color='transparent' shadow={false}>
          <div className='w-full max-w-[20rem] p-4 border-r border-blue-gray-50 overflow-y-auto h-screen [&::-webkit-scrollbar]:hidden [-ms-overflow-style:none] [scrollbar-width:none] select-none'>
            <List>
              <Accordion
                open={expanded}
                icon={
                  <ChevronDownIcon
                    strokeWidth={2.5}
                    className={`mx-auto h-4 w-4 transition-transform ${
                      expanded ? 'rotate-180' : ''
                    }`}
                  />
                }
              >
                <div className='flex flex-row items-center gap-3'>
                  <ListItem className='p-0'>
                    <AccordionHeader
                      onClick={toggleExpand}
                      className='border-b-0 p-3'
                    >
                      <Typography
                        color='blue-gray'
                        className='mr-auto font-normal'
                      >
                        {t('component.sidebar.library')}
                      </Typography>
                    </AccordionHeader>
                  </ListItem>
                </div>
                <AccordionBody className='py-1'>
                  {!isLoading && (
                    <List className='p-0'>
                      {data?.map(mediaLibrary => (
                        <div
                          className='flex flex-row justify-between gap-3'
                          key={mediaLibrary.id.toString()}
                        >
                          <ListItem
                            onClick={() =>
                              navigate(`/library/${mediaLibrary.id}`)
                            }
                          >
                            <ListItemPrefix>
                              {getIconAccordingToCategory(
                                mediaLibrary.category
                              )}
                            </ListItemPrefix>
                            {mediaLibrary.name}
                          </ListItem>
                        </div>
                      ))}
                    </List>
                  )}
                </AccordionBody>
              </Accordion>
            </List>
          </div>
        </Card>
      </Drawer>
    </>
  )
}
