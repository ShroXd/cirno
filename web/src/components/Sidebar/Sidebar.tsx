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

import { getIconAccordingToCategory } from './utils'
import { LibraryDto } from '~/bindings/LibraryDto'
import { useFetch } from '~/hooks/useFetch'

interface SidebarProps {
  open: boolean
  onClose: () => void
}

export const Sidebar = ({ open, onClose }: SidebarProps) => {
  const [expanded, setExpanded] = useState(true)

  const { t } = useTranslation()
  const navigate = useNavigate()

  const { data, isLoading } = useFetch<LibraryDto[]>('/library/')

  const toggleExpand = () => {
    setExpanded(!expanded)
  }

  // const handleDeleteMediaLibraryConfirmation = () => {
  //   mutate(`/library/`)
  //   setShowDeleteConfirmation(false)
  //   setMediaLibraryToDelete(null)
  // }

  // const { listenForMessages } = useEventBus()
  // listenForMessages('media_library_scanned', (payload: unknown) => {
  //   console.log('payload', payload)
  //   // TODO: update state of the specific media library
  //   setIsScanning(false)
  // })

  return (
    <>
      <Drawer
        open={open}
        onClose={onClose}
        aria-label={t('component.sidebar.title')}
      >
        <Card color='transparent' shadow={false}>
          <div className='h-screen w-full max-w-[20rem] select-none overflow-y-auto border-r border-blue-gray-50 p-4 [-ms-overflow-style:none] [scrollbar-width:none] [&::-webkit-scrollbar]:hidden'>
            <List role='navigation'>
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
                <ListItem className='p-0'>
                  <AccordionHeader
                    onClick={toggleExpand}
                    className='border-b-0 p-3'
                    aria-expanded={expanded}
                  >
                    <Typography
                      color='blue-gray'
                      className='mr-auto font-normal'
                    >
                      {t('component.sidebar.library')}
                    </Typography>
                  </AccordionHeader>
                </ListItem>
                <AccordionBody className='py-1'>
                  {!isLoading && (
                    <List className='p-0'>
                      {data?.map(mediaLibrary => (
                        <button
                          key={mediaLibrary.id.toString()}
                          onClick={() =>
                            navigate(`/library/${mediaLibrary.id}`)
                          }
                          className='flex w-full flex-row justify-between gap-3'
                        >
                          <ListItem>
                            <ListItemPrefix>
                              {getIconAccordingToCategory(
                                mediaLibrary.category
                              )}
                            </ListItemPrefix>
                            <Typography
                              color='blue-gray'
                              className='mr-auto font-normal'
                            >
                              {mediaLibrary.name}
                            </Typography>
                          </ListItem>
                        </button>
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
