import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { useNavigate } from 'react-router-dom'

import {
  Bars3Icon,
  InboxStackIcon,
  MagnifyingGlassIcon,
  XMarkIcon,
} from '@heroicons/react/24/outline'
import {
  Button,
  IconButton,
  Input,
  Navbar,
  Typography,
} from '@material-tailwind/react'

import { CreateDialog } from '../LibraryManageDialog/CreateDialog/CreateDialog'
import { Sidebar } from '~/components/Sidebar/Sidebar'

export const StickyNavbar = () => {
  const [open, setOpen] = useState(false)
  const { t } = useTranslation()
  const navigate = useNavigate()
  const [createDialogOpen, setCreateDialogOpen] = useState(false)

  const openSidebar = () => setOpen(true)
  const closeSidebar = () => setOpen(false)

  const createDialogHandler = () => {
    setCreateDialogOpen(!createDialogOpen)
  }

  return (
    <>
      <CreateDialog
        open={createDialogOpen}
        dialogHandler={createDialogHandler}
      />
      <Navbar
        className='fixed left-0 right-0 top-0 z-10 max-w-full rounded-none border-b border-gray-200 bg-gray-50 bg-opacity-85 px-4 py-2 shadow-none backdrop-blur-sm'
        variant='gradient'
      >
        <div className='flex items-center justify-start text-blue-gray-900'>
          <IconButton variant='text' size='lg' onClick={openSidebar}>
            {open ? (
              <XMarkIcon className='h-6 w-6 stroke-2' />
            ) : (
              <Bars3Icon className='h-6 w-6 stroke-2' />
            )}
          </IconButton>
          <Typography
            className='ml-4 cursor-pointer'
            variant='h5'
            color='blue-gray'
            onClick={() => navigate('/')}
          >
            Cirno
          </Typography>
          <div className='ml-auto flex items-center gap-4'>
            <div className='mr-4 hidden lg:block'>
              <ul className='mb-4 mt-2 flex flex-col gap-2 lg:mb-0 lg:mt-0 lg:flex-row lg:items-center lg:gap-6'>
                <Typography
                  as='li'
                  variant='small'
                  color='blue-gray'
                  className='p-1 font-normal'
                >
                  <a href='#/favorites' className='flex items-center'>
                    {t('component.navbar.favorites')}
                  </a>
                </Typography>
                <Typography
                  as='li'
                  variant='small'
                  color='blue-gray'
                  className='p-1 font-normal'
                >
                  <a href='#/settings' className='flex items-center'>
                    {t('component.sidebar.settings')}
                  </a>
                </Typography>
                <div className='w-[16rem] p-2'>
                  <Input
                    icon={<MagnifyingGlassIcon className='h-5 w-5' />}
                    size='md'
                    label='Search'
                  />
                </div>
                <Button
                  variant='gradient'
                  className='flex items-center gap-4'
                  onClick={createDialogHandler}
                >
                  <InboxStackIcon className='h-4 w-4' />
                  {t('component.navbar.addLibrary')}
                </Button>
              </ul>
            </div>
          </div>
        </div>
      </Navbar>
      <Sidebar open={open} onClose={closeSidebar} />
    </>
  )
}
