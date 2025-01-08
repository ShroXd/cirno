import { PhotoIcon } from '@heroicons/react/24/outline'
import { CardBody, CardHeader, Typography } from '@material-tailwind/react'

import { ContentCardContainer } from './Container'

export const ContentCardSkeleton = () => {
  return (
    <ContentCardContainer className='animate-pulse'>
      <CardHeader
        shadow={false}
        floated={false}
        className='relative m-0 mb-5 grid h-64 place-items-center rounded-b-none bg-gray-300'
      >
        <PhotoIcon className='h-12 w-12 text-gray-500' />
      </CardHeader>
      <CardBody className='pb-4 pt-0'>
        <Typography
          as='div'
          variant='h1'
          className='h-3 rounded-full bg-gray-300'
        >
          &nbsp;
        </Typography>
      </CardBody>
    </ContentCardContainer>
  )
}
