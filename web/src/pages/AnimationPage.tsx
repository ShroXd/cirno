import { useEffect, useState } from 'react'
import { Link } from 'react-router-dom'

import {
  Calendar,
  ChevronRight,
  Clock,
  Film,
  Filter,
  Play,
  Plus,
  Sparkles,
  Star,
  Tv,
} from 'lucide-react'

import { Badge } from '../components/ui/badge'
import { Button } from '../components/ui/button'
import { Card, CardContent } from '../components/ui/card'
import { Skeleton } from '../components/ui/skeleton'
import { Tabs, TabsList, TabsTrigger } from '../components/ui/tabs'
import { cn } from '../lib/utils'

// Mock library content (replace with actual data fetching)
const libraryContent = [
  {
    id: 'animation-1',
    title: 'Adventure Time',
    type: 'tv',
    genres: ['Animation', 'Adventure', 'Comedy'],
    image: '/placeholder.svg?height=300&width=200',
    year: 2010,
    seasons: 10,
    rating: 8.6,
  },
  {
    id: 'animation-2',
    title: 'Spirited Away',
    type: 'movie',
    genres: ['Animation', 'Adventure', 'Family'],
    image: '/placeholder.svg?height=300&width=200',
    year: 2001,
    duration: '2h 5m',
    rating: 8.6,
  },
  {
    id: 'animation-3',
    title: 'The Simpsons',
    type: 'tv',
    genres: ['Animation', 'Comedy'],
    image: '/placeholder.svg?height=300&width=200',
    year: 1989,
    seasons: 34,
    rating: 8.7,
  },
  {
    id: 'animation-4',
    title: 'Toy Story',
    type: 'movie',
    genres: ['Animation', 'Family', 'Comedy', 'Adventure'],
    image: '/placeholder.svg?height=300&width=200',
    year: 1995,
    duration: '1h 21m',
    rating: 8.3,
  },
  {
    id: 'animation-5',
    title: 'Avatar: The Last Airbender',
    type: 'tv',
    genres: ['Animation', 'Action', 'Adventure', 'Family'],
    image: '/placeholder.svg?height=300&width=200',
    year: 2005,
    seasons: 3,
    rating: 9.3,
  },
  {
    id: 'animation-6',
    title: 'Spider-Man: Into the Spider-Verse',
    type: 'movie',
    genres: ['Animation', 'Action', 'Adventure'],
    image: '/placeholder.svg?height=300&width=200',
    year: 2018,
    duration: '1h 57m',
    rating: 8.4,
  },
]

// Filter animated content from library content
const animatedContent = libraryContent.filter(item =>
  item.genres.includes('Animation')
)

// Animation categories
const animationCategories = [
  {
    id: 'animated-movies',
    title: 'Animated Movies',
    items: animatedContent.filter(item => item.type === 'movie'),
  },
  {
    id: 'animated-series',
    title: 'Animated Series',
    items: animatedContent.filter(item => item.type === 'tv'),
  },
  {
    id: 'family-friendly',
    title: 'Family Friendly',
    items: animatedContent.filter(
      item =>
        item.genres.includes('Comedy') ||
        item.genres.includes('Family') ||
        item.genres.includes('Adventure')
    ),
  },
  {
    id: 'action-animation',
    title: 'Action Animation',
    items: animatedContent.filter(item => item.genres.includes('Action')),
  },
]

// Featured animation
const featuredAnimation = {
  id: 'featured-animation',
  title: 'Spider-Man: Across the Spider-Verse',
  description:
    'Miles Morales catapults across the Multiverse, where he encounters a team of Spider-People charged with protecting its very existence. When the heroes clash on how to handle a new threat, Miles must redefine what it means to be a hero.',
  image: '/placeholder.svg?height=600&width=1200',
  type: 'movie',
  year: 2023,
  rating: 8.7,
  duration: '2h 20m',
  genres: ['Animation', 'Action', 'Adventure'],
  director: 'Joaquim Dos Santos, Kemp Powers, Justin K. Thompson',
  cast: ['Shameik Moore', 'Hailee Steinfeld', 'Oscar Isaac'],
}

// Studios
const animationStudios = [
  {
    id: 'pixar',
    name: 'Pixar',
    logo: '/placeholder.svg?height=100&width=100',
    color: 'bg-blue-500/10 text-blue-500',
  },
  {
    id: 'dreamworks',
    name: 'DreamWorks',
    logo: '/placeholder.svg?height=100&width=100',
    color: 'bg-green-500/10 text-green-500',
  },
  {
    id: 'ghibli',
    name: 'Studio Ghibli',
    logo: '/placeholder.svg?height=100&width=100',
    color: 'bg-yellow-500/10 text-yellow-500',
  },
  {
    id: 'disney',
    name: 'Disney',
    logo: '/placeholder.svg?height=100&width=100',
    color: 'bg-purple-500/10 text-purple-500',
  },
  {
    id: 'sony',
    name: 'Sony Pictures Animation',
    logo: '/placeholder.svg?height=100&width=100',
    color: 'bg-red-500/10 text-red-500',
  },
  {
    id: 'cartoon-network',
    name: 'Cartoon Network',
    logo: '/placeholder.svg?height=100&width=100',
    color: 'bg-orange-500/10 text-orange-500',
  },
]

export default function AnimationPage() {
  const [isLoading, setIsLoading] = useState(true)
  const [activeTab, setActiveTab] = useState('all')

  // Simulate loading state
  useEffect(() => {
    const timer = setTimeout(() => {
      setIsLoading(false)
    }, 1000)

    return () => clearTimeout(timer)
  }, [])

  if (isLoading) {
    return <AnimationPageSkeleton />
  }

  return (
    <div className='container mx-auto px-4 py-6 md:px-6'>
      <div className='mb-6 flex items-center justify-between'>
        <h1 className='flex items-center text-3xl font-bold'>
          <Sparkles className='mr-2 h-6 w-6 text-yellow-400' /> Animation
        </h1>
        <Button variant='outline' size='sm' className='gap-2'>
          <Filter className='h-4 w-4' /> Filter
        </Button>
      </div>

      {/* Featured Animation */}
      <div className='animate-fade-in relative mb-12 h-[50vh] w-full overflow-hidden rounded-xl'>
        <img
          src={featuredAnimation.image || '/placeholder.svg'}
          alt={featuredAnimation.title}
          className='absolute inset-0 h-full w-full object-cover'
        />
        <div className='absolute inset-0 bg-gradient-to-t from-background via-background/80 to-transparent' />

        <div className='absolute bottom-0 left-0 right-0 p-6 md:p-8'>
          <div className='flex max-w-3xl flex-col gap-4'>
            <Badge
              variant='secondary'
              className='w-fit bg-yellow-500/20 text-yellow-500'
            >
              <Sparkles className='mr-1 h-3 w-3' /> Featured Animation
            </Badge>

            <h2 className='text-3xl font-bold md:text-5xl'>
              {featuredAnimation.title}
            </h2>
            <p className='max-w-2xl text-muted-foreground'>
              {featuredAnimation.description}
            </p>

            <div className='flex flex-wrap items-center gap-4 text-sm text-muted-foreground'>
              <div className='flex items-center'>
                <Star className='mr-1 h-4 w-4 fill-yellow-400 text-yellow-400' />
                <span>{featuredAnimation.rating}/10</span>
              </div>
              <div className='flex items-center'>
                <Calendar className='mr-1 h-4 w-4' />
                <span>{featuredAnimation.year}</span>
              </div>
              <div className='flex items-center'>
                <Clock className='mr-1 h-4 w-4' />
                <span>{featuredAnimation.duration}</span>
              </div>
            </div>

            <div className='mt-2 flex gap-3'>
              <Button className='gap-2' asChild>
                <Link to={`/content/${featuredAnimation.id}`}>
                  <Play className='h-4 w-4' /> Watch Now
                </Link>
              </Button>
              <Button variant='outline' className='gap-2'>
                <Plus className='h-4 w-4' /> Add to Playlist
              </Button>
            </div>
          </div>
        </div>
      </div>

      {/* Animation Studios */}
      <section className='animate-fade-in mb-12'>
        <h2 className='mb-6 text-2xl font-semibold'>Animation Studios</h2>
        <div className='grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-6'>
          {animationStudios.map(studio => (
            <Link to={`/studio/${studio.id}`} key={studio.id}>
              <Card
                className={cn(
                  'group flex h-32 flex-col items-center justify-center overflow-hidden text-center transition-all duration-300 hover:shadow-md',
                  studio.color
                )}
              >
                <CardContent className='flex w-full flex-col items-center p-4'>
                  <div className='mb-2 h-12 w-12 overflow-hidden rounded-full'>
                    <img
                      src={studio.logo || '/placeholder.svg'}
                      alt={studio.name}
                      className='h-full w-full object-cover'
                    />
                  </div>
                  <h3 className='text-sm font-semibold'>{studio.name}</h3>
                </CardContent>
              </Card>
            </Link>
          ))}
        </div>
      </section>

      {/* Animation Categories */}
      <Tabs
        defaultValue='all'
        value={activeTab}
        onValueChange={setActiveTab}
        className='mb-6'
      >
        <TabsList>
          <TabsTrigger value='all'>All Animation</TabsTrigger>
          <TabsTrigger value='movies'>Animated Movies</TabsTrigger>
          <TabsTrigger value='series'>Animated Series</TabsTrigger>
          <TabsTrigger value='family'>Family Friendly</TabsTrigger>
        </TabsList>
      </Tabs>

      {activeTab === 'all' ? (
        // Show all categories when "all" is selected
        <div className='animate-fade-in space-y-12'>
          {animationCategories.map(category => (
            <section key={category.id}>
              <div className='mb-4 flex items-center justify-between'>
                <h2 className='text-2xl font-semibold'>{category.title}</h2>
                <Button variant='ghost' size='sm' className='gap-1' asChild>
                  <Link to={`/view-all?category=all&title=${category.title}`}>
                    See All <ChevronRight className='h-4 w-4' />
                  </Link>
                </Button>
              </div>

              <div className='grid grid-cols-2 gap-6 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5'>
                {category.items.map(item => (
                  <AnimationCard key={item.id} item={item} />
                ))}
              </div>
            </section>
          ))}
        </div>
      ) : (
        // Show filtered animation when a specific category is selected
        <div className='animate-fade-in'>
          <div className='grid grid-cols-2 gap-6 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5'>
            {animatedContent
              .filter(item => {
                if (activeTab === 'movies') return item.type === 'movie'
                if (activeTab === 'series') return item.type === 'tv'
                if (activeTab === 'family')
                  return (
                    item.genres.includes('Comedy') ||
                    item.genres.includes('Family') ||
                    item.genres.includes('Adventure')
                  )
                return true
              })
              .map(item => (
                <AnimationCard key={item.id} item={item} />
              ))}
          </div>
        </div>
      )}
    </div>
  )
}

// Animation Card Component
function AnimationCard({ item }: { item: any }) {
  return (
    <Card className='group overflow-hidden transition-all duration-300 hover:shadow-lg'>
      <Link to={`/content/${item.id}`}>
        <div className='relative aspect-[2/3]'>
          <img
            src={item.image || '/placeholder.svg'}
            alt={item.title}
            className='absolute inset-0 h-full w-full object-cover transition-transform duration-500 group-hover:scale-105'
          />
          <div className='absolute right-2 top-2'>
            {item.type === 'movie' ? (
              <Badge
                variant='secondary'
                className='bg-blue-500/20 text-blue-500'
              >
                <Film className='mr-1 h-3 w-3' /> Movie
              </Badge>
            ) : (
              <Badge
                variant='secondary'
                className='bg-purple-500/20 text-purple-500'
              >
                <Tv className='mr-1 h-3 w-3' /> TV
              </Badge>
            )}
          </div>
          <div className='absolute inset-0 flex items-end bg-gradient-to-t from-black/80 to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100'>
            <div className='w-full p-4'>
              <Button size='sm' variant='secondary' className='w-full gap-2'>
                <Play className='h-4 w-4' /> Watch Now
              </Button>
            </div>
          </div>
        </div>
        <CardContent className='p-3'>
          <div className='flex items-start justify-between'>
            <div>
              <h3 className='line-clamp-1 font-medium'>{item.title}</h3>
              <p className='text-sm text-muted-foreground'>
                {item.type === 'movie'
                  ? `${item.year} • ${item.duration}`
                  : `${item.year} • ${item.seasons} Seasons`}
              </p>
            </div>
            <Badge variant='outline' className='ml-2 bg-primary/10'>
              {item.rating}
            </Badge>
          </div>
          <div className='mt-2 flex flex-wrap gap-1'>
            {item.genres
              .filter((genre: string) => genre !== 'Animation')
              .slice(0, 2)
              .map((genre: string) => (
                <Badge key={genre} variant='secondary' className='text-xs'>
                  {genre}
                </Badge>
              ))}
          </div>
        </CardContent>
      </Link>
    </Card>
  )
}

// Loading Skeleton
function AnimationPageSkeleton() {
  return (
    <div className='container mx-auto px-4 py-6 md:px-6'>
      <div className='mb-6 flex items-center justify-between'>
        <Skeleton className='h-10 w-40' />
        <Skeleton className='h-9 w-24' />
      </div>

      {/* Featured Animation Skeleton */}
      <Skeleton className='mb-12 h-[50vh] w-full rounded-xl' />

      {/* Studios Skeleton */}
      <div className='mb-12'>
        <Skeleton className='mb-6 h-8 w-48' />
        <div className='grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-6'>
          {[1, 2, 3, 4, 5, 6].map(i => (
            <Skeleton key={i} className='h-32 rounded-lg' />
          ))}
        </div>
      </div>

      {/* Tabs Skeleton */}
      <Skeleton className='mb-6 h-10 w-96' />

      {/* Categories Skeleton */}
      {[1, 2, 3, 4].map(category => (
        <div key={category} className='mb-12'>
          <div className='mb-4 flex items-center justify-between'>
            <Skeleton className='h-8 w-48' />
            <Skeleton className='h-8 w-24' />
          </div>
          <div className='grid grid-cols-2 gap-6 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5'>
            {[1, 2, 3, 4, 5].map(i => (
              <div key={i} className='flex flex-col space-y-3'>
                <Skeleton className='aspect-[2/3] rounded-md' />
                <Skeleton className='h-5 w-full' />
                <Skeleton className='h-4 w-2/3' />
                <Skeleton className='h-4 w-1/2' />
              </div>
            ))}
          </div>
        </div>
      ))}
    </div>
  )
}
