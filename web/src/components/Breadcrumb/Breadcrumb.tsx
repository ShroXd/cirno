import { HomeIcon } from '@heroicons/react/24/outline'
import { Breadcrumbs } from '@material-tailwind/react'
import { useTranslation } from 'react-i18next'
import { Link, useLocation } from 'react-router-dom'

export const Breadcrumb = () => {
  const location = useLocation()
  const { t } = useTranslation()

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

  // const handleGoBack = () => {
  //   if (pathSegmentsView.length > 1) {
  //     const parentPath = '/' + pathSegments.slice(0, -2).join('/')
  //     navigate(parentPath)
  //   } else {
  //     navigate('/')
  //   }
  // }

  return (
    <div className='flex flex-row items-center justify-start gap-2'>
      {/* <Button
        onClick={handleGoBack}
        className='flex items-center text-sm rounded-full'
        variant='text'
        ripple={false}
        size='sm'
      >
        <ChevronLeftIcon className='h-4 w-4' />
      </Button> */}
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
