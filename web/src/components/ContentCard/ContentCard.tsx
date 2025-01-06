import { FC } from 'react'

import { Typography } from '@material-tailwind/react'

interface CardProps {
  imageUrl: string
  title: string
}

export const ContentCard: FC<CardProps> = ({ imageUrl, title }) => (
  <div className='flex select-none flex-col overflow-hidden rounded-xl bg-white shadow-lg'>
    <div className='overflow-hidden'>
      <img
        src={imageUrl}
        alt={title}
        className='h-64 w-full object-cover transition-all duration-700 ease-[linear(0,_0.402_7.4%,_0.711_15.3%,_0.929_23.7%,_1.008_28.2%,_1.067_33%,_1.099_36.9%,_1.12_41%,_1.13_45.4%,_1.13_50.1%,_1.111_58.5%,_1.019_83.2%,_1.004_91.3%,_1)] hover:scale-105'
      />
    </div>
    <div className='bg-white px-4 py-3'>
      <Typography
        variant='paragraph'
        className='truncate text-center font-medium'
      >
        {title}
      </Typography>
    </div>
  </div>
)
