import { Link } from 'react-router-dom'

import { Play, Plus } from 'lucide-react'

import { Button } from '../ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '../ui/card'

interface MediaCardProps {
  id: bigint
  title: string
  posterPath: string
  plot: string
}

export const MediaCard = ({ id, title, posterPath, plot }: MediaCardProps) => {
  return (
    <Card key={title} className='overflow-hidden'>
      <div className='relative h-48 w-full'>
        <Link to={`/library/${id}`}>
          <div className='ease-[cubic-bezier(0.4,0,0.2,1)] relative h-48 w-full overflow-hidden object-cover transition-all duration-700 hover:scale-105'>
            <img
              src={posterPath || '/placeholder.svg'}
              alt={title}
              className='absolute inset-0 h-full w-full object-cover'
            />
            <div className='absolute inset-0 bg-gradient-to-t from-black/80 to-transparent' />
          </div>
          <div className='absolute bottom-3 left-3 flex gap-2'>
            <Button size='sm' variant='secondary' className='rounded-full'>
              <Play className='mr-1 h-4 w-4' /> Play
            </Button>
            <Button
              size='icon'
              variant='outline'
              className='h-8 w-8 rounded-full'
            >
              <Plus className='h-4 w-4' />
            </Button>
          </div>
        </Link>
      </div>
      <CardHeader className='p-3 pb-0'>
        <Link to={`/library/${id}`}>
          <CardTitle className='text-lg'>{title}</CardTitle>
        </Link>
      </CardHeader>
      <CardContent className='p-3 pt-1'>
        <CardDescription className='line-clamp-2 text-xs'>
          {plot}
        </CardDescription>
      </CardContent>
      <CardFooter className='p-3 pt-0 text-xs text-muted-foreground'>
        {'Movie'}
      </CardFooter>
    </Card>
  )
}
