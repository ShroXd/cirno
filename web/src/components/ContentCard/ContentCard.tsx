import { Typography } from '@material-tailwind/react'
import { FC } from 'react'

interface CardProps {
  imageUrl: string
  title: string
}

export const ContentCard: FC<CardProps> = ({ imageUrl, title }) => (
  <div className='bg-white flex flex-col select-none overflow-hidden shadow-lg rounded-xl'>
    <div className='overflow-hidden'>
      <img
        src={imageUrl}
        alt={title}
        className='w-full h-64 object-cover hover:scale-105 transition-all duration-700 ease-[linear(0,_0.402_7.4%,_0.711_15.3%,_0.929_23.7%,_1.008_28.2%,_1.067_33%,_1.099_36.9%,_1.12_41%,_1.13_45.4%,_1.13_50.1%,_1.111_58.5%,_1.019_83.2%,_1.004_91.3%,_1)]'
      />
    </div>
    <div className='px-4 py-3 bg-white'>
      <Typography
        variant='paragraph'
        className='text-center font-medium truncate'
      >
        {title}
      </Typography>
    </div>
  </div>
)
