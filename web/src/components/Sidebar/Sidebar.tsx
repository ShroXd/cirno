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
} from '@heroicons/react/24/solid'
import { MagnifyingGlassIcon } from '@heroicons/react/24/outline'
import { useNavigate } from 'react-router-dom'
import { useTranslation } from 'react-i18next'
import { useState } from 'react'

export const Sidebar = () => {
  const [open, setOpen] = useState(0)

  const navigate = useNavigate()
  const { t } = useTranslation()

  const toggleOpen = (value: number) => {
    setOpen(open === value ? 0 : value)
  }

  return (
    <div className='w-full max-w-[20rem] p-4 border-r border-blue-gray-50'>
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
          open={open === 1}
          icon={
            <ChevronDownIcon
              strokeWidth={2.5}
              className={`mx-auto h-4 w-4 transition-transform ${open === 1 ? 'rotate-180' : ''}`}
            />
          }
        >
          <div className='flex flex-row items-center gap-3'>
            <ListItem className='p-0' onClick={() => navigate('/')}>
              <AccordionHeader
                onClick={() => toggleOpen(1)}
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
            <Button className='p-3 !overflow-visible' variant='text'>
              <PlusIcon className='h-4 w-4' />
            </Button>
          </div>
          <AccordionBody className='py-1'>
            <List className='p-0'>
              <ListItem className='pl-6'>
                AAnalyticsAnalyticsAnalyticsnalytics
              </ListItem>
              <ListItem className='pl-6'>Reporting</ListItem>
              <ListItem className='pl-6'>Projects</ListItem>
            </List>
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
  )
}
