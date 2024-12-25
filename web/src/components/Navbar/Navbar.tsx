import { Navbar, Typography, IconButton, Input } from '@material-tailwind/react'
import { Sidebar } from '../Sidebar/Sidebar'
import { useState } from 'react'
import { MagnifyingGlassIcon, XMarkIcon } from '@heroicons/react/24/outline'
import { Bars3Icon } from '@heroicons/react/24/outline'
import { useTranslation } from 'react-i18next'
import { useNavigate } from 'react-router-dom'

export const StickyNavbar = () => {
  const [open, setOpen] = useState(false)
  const { t } = useTranslation()
  const navigate = useNavigate()

  const openSidebar = () => setOpen(true)
  const closeSidebar = () => setOpen(false)

  return (
    <>
      <Navbar className='fixed top-0 left-0 right-0 z-10 max-w-full bg-white bg-opacity-70 backdrop-blur-sm border-b border-gray-200 px-4 py-2'>
        <div className='flex items-center justify-start  text-blue-gray-900'>
          <IconButton variant='text' size='lg' onClick={openSidebar}>
            {open ? (
              <XMarkIcon className='h-8 w-8 stroke-2' />
            ) : (
              <Bars3Icon className='h-8 w-8 stroke-2' />
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
          <div className='flex items-center ml-auto gap-4'>
            <div className='mr-4 hidden lg:block'>
              <ul className='mt-2 mb-4 flex flex-col gap-2 lg:mb-0 lg:mt-0 lg:flex-row lg:items-center lg:gap-6'>
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
                <div className='p-2 w-[16rem]'>
                  <Input
                    icon={<MagnifyingGlassIcon className='h-5 w-5' />}
                    size='md'
                    label='Search'
                  />
                </div>
              </ul>
            </div>
          </div>
        </div>
      </Navbar>
      <Sidebar open={open} onClose={closeSidebar} />
    </>
  )
}
