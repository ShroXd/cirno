import { Film, Play, Tv } from 'lucide-react'
import { motion } from 'motion/react'

import { Button } from '../ui/button'
import { Card } from '../ui/card'

interface VerticalCardProps {
  title: string
  posterPath: string
  plot: string
  category: 'movie' | 'tv'
  year: string
  duration?: string
  episodeCount?: number
}

export const VerticalCard = ({
  title,
  posterPath,
  category,
  year,
  duration,
  episodeCount,
}: VerticalCardProps) => {
  const CategoryIcon = category === 'movie' ? Film : Tv
  const categoryText = category === 'movie' ? '电影' : '电视剧'
  const detailText =
    category === 'movie'
      ? `${year} · ${duration || ''}`
      : `${year} · ${episodeCount || 0} 集`

  return (
    <motion.div
      whileHover={{ y: -5 }}
      transition={{ type: 'spring', stiffness: 300 }}
    >
      <Card className='group h-[300px] w-[200px] cursor-pointer overflow-hidden'>
        <div className='relative h-full w-full'>
          <img
            src={posterPath}
            alt={title}
            className='h-full w-full object-cover transition-transform duration-300 group-hover:scale-105'
          />

          <div className='absolute inset-0 flex flex-col justify-between bg-gradient-to-t from-black/80 via-black/40 to-transparent p-3 opacity-0 transition-opacity duration-300 group-hover:opacity-100'>
            <div></div>
            <div className='flex justify-center'>
              <Button
                size='icon'
                variant='secondary'
                className='mt-4 scale-90 transform rounded-full opacity-80'
              >
                <Play className='h-4 w-4' />
              </Button>
            </div>

            <div className='translate-y-2 transform space-y-1 transition-transform duration-300 group-hover:translate-y-0'>
              <div className='flex items-center gap-1 text-xs text-white/80'>
                <CategoryIcon className='h-3.5 w-3.5' />
                <span>{categoryText}</span>
              </div>
              <h4 className='line-clamp-1 text-sm font-medium text-white'>
                {title}
              </h4>
              <p className='text-xs text-white/70'>{detailText}</p>
            </div>
          </div>
        </div>
      </Card>
    </motion.div>
  )
}
