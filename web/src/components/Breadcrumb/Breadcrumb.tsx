import { HomeIcon } from '@heroicons/react/24/outline'
import { ChevronLeftIcon } from '@heroicons/react/24/solid'
import { Breadcrumbs, Button } from '@material-tailwind/react'
import { Link, useLocation, useNavigate } from 'react-router-dom'

export const Breadcrumb = () => {
  const location = useLocation()
  const navigate = useNavigate()

  const pathSegments = location.pathname.split('/').filter(Boolean)
  const pathSegmentsIds = pathSegments.filter(
    segment => !isNaN(Number(segment))
  )
  const pathSegmentsView = pathSegments.filter(segment =>
    isNaN(Number(segment))
  )

  const getBreadcrumbPath = (index: number) => {
    const path = `/${pathSegments.slice(0, index + 1).join('/')}/${pathSegmentsIds[index]}`
    return {
      to: path,
    }
  }

  const handleGoBack = () => {
    if (pathSegmentsView.length > 1) {
      const parentPath = '/' + pathSegments.slice(0, -2).join('/')
      navigate(parentPath)
    } else {
      navigate('/')
    }
  }

  return (
    <div className='flex flex-row items-center justify-start gap-2'>
      <Button
        onClick={handleGoBack}
        className='flex items-center gap-2 text-sm text-blue-gray-500 hover:text-blue-gray-700'
        variant='text'
        ripple={false}
        size='sm'
      >
        <ChevronLeftIcon className='h-4 w-4' />
      </Button>
      <Breadcrumbs>
        <Link to={{ pathname: '/' }} className='opacity-60'>
          <HomeIcon className='h-4 w-4' />
        </Link>
        {pathSegmentsView.map((segment, index) => (
          <Link
            key={segment}
            to={getBreadcrumbPath(index).to}
            className='opacity-60'
          >
            <span>{segment}</span>
          </Link>
        ))}
      </Breadcrumbs>
    </div>
  )
}

export default Breadcrumb
