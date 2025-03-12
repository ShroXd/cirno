import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { useNavigate } from 'react-router-dom'

import {
  IconButton,
  Input,
  Navbar,
  Typography,
} from '@material-tailwind/react'
import { Inbox, Menu, Search, X } from 'lucide-react';

import { CreateDialog } from '../LibraryManageDialog/CreateDialog/CreateDialog'
import { Sidebar } from '~/components/Sidebar/Sidebar'
import { Button } from '../ui/button'

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
        aria-label={t('component.stickyNavbar.title')}
      >
        <div className='flex items-center justify-start text-blue-gray-900'>
          <IconButton
            aria-label={t('component.stickyNavbar.toggleSidebar')}
            variant='text'
            size='lg'
            onClick={openSidebar}
          >
            {open ? (
              <X className='h-6 w-6 stroke-2' />
            ) : (
              <Menu className='h-6 w-6 stroke-2' />
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
          <div className='mb-4 ml-auto mt-2 flex flex-col gap-2 lg:mb-0 lg:mt-0 lg:flex-row lg:items-center lg:gap-6'>
            <Typography
              variant='small'
              color='blue-gray'
              className='p-1 font-normal'
            >
              <a href='#/favorites' className='flex items-center'>
                {t('component.stickyNavbar.favorites')}
              </a>
            </Typography>
            <Typography
              variant='small'
              color='blue-gray'
              className='p-1 font-normal'
            >
              <a href='#/settings' className='flex items-center'>
                {t('component.stickyNavbar.settings')}
              </a>
            </Typography>
            <div className='w-[16rem] p-2'>
              <Input
                icon={<Search className='h-5 w-5' />}
                size='md'
                aria-label={t('component.stickyNavbar.search')}
              />
            </div>
            <Button
              aria-label={t('component.stickyNavbar.addLibrary')}
              className='flex items-center gap-4'
              onClick={createDialogHandler}
            >
              <Inbox className='h-4 w-4' />
              {t('component.stickyNavbar.addLibrary')}
            </Button>
          </div>
        </div>
      </Navbar>
      <Sidebar open={open} onClose={closeSidebar} />
    </>
  )
}
