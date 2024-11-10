import {
  Typography,
  List,
  ListItem,
  ListItemPrefix,
  Input,
} from '@material-tailwind/react'
import {
  Cog6ToothIcon,
  LightBulbIcon,
  InboxStackIcon,
} from '@heroicons/react/24/solid'
import { MagnifyingGlassIcon } from '@heroicons/react/24/outline'
import { useNavigate } from 'react-router-dom'
import { useTranslation } from 'react-i18next'

export const Sidebar = () => {
  const navigate = useNavigate()
  const { t } = useTranslation()

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
        <ListItem onClick={() => navigate('/')}>
          <ListItemPrefix>
            <InboxStackIcon className='h-5 w-5' />
          </ListItemPrefix>
          <Typography color='blue-gray' className='mr-auto font-normal'>
            {t('sidebar.library')}
          </Typography>
        </ListItem>
        <hr className='my-2 border-blue-gray-50' />
        <ListItem onClick={() => navigate('/settings')}>
          <ListItemPrefix>
            <Cog6ToothIcon className='h-5 w-5' />
          </ListItemPrefix>
          {t('sidebar.settings')}
        </ListItem>
        <ListItem onClick={() => navigate('/test')}>
          <ListItemPrefix>
            <LightBulbIcon className='h-5 w-5' />
          </ListItemPrefix>
          {t('sidebar.test')}
        </ListItem>
      </List>
    </div>
  )
}
