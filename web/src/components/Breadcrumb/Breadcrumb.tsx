import { useTranslation } from 'react-i18next'
import { Link, useLocation, useNavigate } from 'react-router-dom'

import { ChevronLeftIcon, HomeIcon } from '@heroicons/react/24/outline'
import { Breadcrumbs, Button } from '@material-tailwind/react'

interface BreadcrumbProps {
  onBack?: () => void
}

export const Breadcrumb = ({ onBack }: BreadcrumbProps) => {
  const location = useLocation()
  const { t } = useTranslation()
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
    console.log('pathSegmentsView', pathSegmentsView)
    if (pathSegmentsView.length > 1) {
      const parentPath = '/' + pathSegments.slice(0, -2).join('/')
      onBack?.()
      navigate(parentPath)
    } else {
      onBack?.()
      navigate('/')
    }
  }

  return (
    <div className='flex flex-row items-center justify-start gap-2'>
      <Button
        onClick={handleGoBack}
        className='flex items-center bg-gray-200 text-sm'
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
            <span>{t(`component.breadcrumb.${segment.toLowerCase()}`)}</span>
          </Link>
        ))}
      </Breadcrumbs>
    </div>
  )
}

export default Breadcrumb
