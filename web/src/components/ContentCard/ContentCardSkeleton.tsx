import { CardBody, CardHeader, Typography } from '@material-tailwind/react'

export const ContentCardSkeleton = () => {
  return (
    <div className='flex animate-pulse select-none flex-col overflow-hidden rounded-xl bg-white shadow-lg'>
      <CardHeader
        shadow={false}
        floated={false}
        className='relative mx-3 my-2 mb-4 grid h-60 place-items-center bg-gray-300'
      >
        <svg
          xmlns='http://www.w3.org/2000/svg'
          fill='none'
          viewBox='0 0 24 24'
          strokeWidth={2}
          stroke='currentColor'
          className='h-12 w-full text-gray-500'
        >
          <path
            strokeLinecap='round'
            strokeLinejoin='round'
            d='m2.25 15.75 5.159-5.159a2.25 2.25 0 0 1 3.182 0l5.159 5.159m-1.5-1.5 1.409-1.409a2.25 2.25 0 0 1 3.182 0l2.909 2.909m-18 3.75h16.5a1.5 1.5 0 0 0 1.5-1.5V6a1.5 1.5 0 0 0-1.5-1.5H3.75A1.5 1.5 0 0 0 2.25 6v12a1.5 1.5 0 0 0 1.5 1.5Zm10.5-11.25h.008v.008h-.008V8.25Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z'
          />
        </svg>
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
    </div>
  )
}
