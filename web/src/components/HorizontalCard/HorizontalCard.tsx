import { Play } from 'lucide-react'
import { motion } from 'motion/react'

import { Button } from '../ui/button'
import { Card, CardContent } from '../ui/card'
import { Progress } from '../ui/progress'

interface HorizontalCardProps {
  item: {
    id: number
    title: string
    thumbnail: string
    progress: number
    lastWatched: string
    remainingTime: string
  }
}

export const HorizontalCard = ({ item }: HorizontalCardProps) => {
  return (
    <motion.div
      key={item.id}
      whileHover={{ y: -5 }}
      transition={{ type: 'spring', stiffness: 300 }}
      className='w-[300px] flex-shrink-0'
    >
      <Card className='overflow-hidden'>
        <div className='relative aspect-video'>
          <img
            src={item.thumbnail || '/placeholder.svg'}
            alt={item.title}
            className='object-cover'
          />
          <div className='absolute inset-0 flex items-center justify-center bg-black/40 opacity-0 transition-opacity hover:opacity-100'>
            <Button size='icon' variant='secondary' className='rounded-full'>
              <Play className='h-6 w-6' />
            </Button>
          </div>
          <div className='absolute bottom-0 left-0 right-0'>
            <Progress value={item.progress} className='h-1 rounded-none' />
          </div>
        </div>
        <CardContent className='p-3'>
          <h3 className='line-clamp-1 font-medium'>{item.title}</h3>
          <div className='mt-1 flex items-center justify-between text-xs text-muted-foreground'>
            <span>{item.lastWatched}</span>
            <span>{item.remainingTime}</span>
          </div>
        </CardContent>
      </Card>
    </motion.div>
  )
}
