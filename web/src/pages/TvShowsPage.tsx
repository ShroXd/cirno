import { useEffect, useState } from 'react'
import { Link } from 'react-router-dom'

import {
  Calendar,
  ChevronRight,
  Filter,
  Play,
  Plus,
  Star,
  Tv,
} from 'lucide-react'

import { Badge } from '../components/ui/badge'
import { Button } from '../components/ui/button'
import { Card, CardContent } from '../components/ui/card'
import { Skeleton } from '../components/ui/skeleton'
import { Tabs, TabsList, TabsTrigger } from '../components/ui/tabs'

// Mock library content (replace with your actual data source)
const libraryContent = [
  {
    id: 'tv-show-1',
    type: 'tv',
    title: 'The Last Kingdom',
    image: '/placeholder.svg?height=300&width=200',
    year: '2015-2022',
    rating: 8.5,
    seasons: 5,
    genres: ['Action', 'Drama', 'History'],
  },
  {
    id: 'tv-show-2',
    type: 'tv',
    title: 'Stranger Things',
    image: '/placeholder.svg?height=300&width=200',
    year: '2016-Present',
    rating: 8.7,
    seasons: 4,
    genres: ['Drama', 'Fantasy', 'Horror'],
  },
  {
    id: 'tv-show-3',
    type: 'tv',
    title: 'Arcane',
    image: '/placeholder.svg?height=300&width=200',
    year: '2021-Present',
    rating: 9.1,
    seasons: 1,
    genres: ['Animation', 'Action', 'Adventure'],
  },
  {
    id: 'tv-show-4',
    type: 'tv',
    title: 'The Witcher',
    image: '/placeholder.svg?height=300&width=200',
    year: '2019-Present',
    rating: 8.2,
    seasons: 3,
    genres: ['Fantasy', 'Action', 'Adventure'],
  },
  {
    id: 'tv-show-5',
    type: 'tv',
    title: 'Avatar: The Last Airbender',
    image: '/placeholder.svg?height=300&width=200',
    year: '2005-2008',
    rating: 9.3,
    seasons: 3,
    genres: ['Animation', 'Action', 'Adventure', 'Fantasy'],
  },
  {
    id: 'tv-show-6',
    type: 'tv',
    title: "The Queen's Gambit",
    image: '/placeholder.svg?height=300&width=200',
    year: '2020',
    rating: 8.6,
    seasons: 1,
    genres: ['Drama'],
  },
]

// Filter TV shows from library content
const tvShows = libraryContent.filter(item => item.type === 'tv')

// TV show categories
const tvCategories = [
  {
    id: 'drama',
    title: 'Drama',
    items: tvShows.filter(show => show.genres.includes('Drama')),
  },
  {
    id: 'action-adventure',
    title: 'Action & Adventure',
    items: tvShows.filter(
      show =>
        show.genres.includes('Action') || show.genres.includes('Adventure')
    ),
  },
  {
    id: 'animation',
    title: 'Animation',
    items: tvShows.filter(show => show.genres.includes('Animation')),
  },
  {
    id: 'fantasy-scifi',
    title: 'Fantasy & Sci-Fi',
    items: tvShows.filter(
      show => show.genres.includes('Fantasy') || show.genres.includes('Sci-Fi')
    ),
  },
]

// Featured TV show
const featuredShow = {
  id: 'featured-show',
  title: 'The Last of Us',
  description:
    "After a global pandemic destroys civilization, a hardened survivor takes charge of a 14-year-old girl who may be humanity's last hope.",
  image: '/placeholder.svg?height=600&width=1200',
  year: '2023-Present',
  rating: 8.8,
  seasons: 1,
  episodes: 9,
  genres: ['Drama', 'Action', 'Horror'],
  creator: 'Craig Mazin, Neil Druckmann',
  cast: ['Pedro Pascal', 'Bella Ramsey', 'Gabriel Luna'],
}

export default function TvShowsPage() {
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
    return <TvShowsPageSkeleton />
  }

  return (
    <div className='container mx-auto px-4 py-6 md:px-6'>
      <div className='mb-6 flex items-center justify-between'>
        <h1 className='text-3xl font-bold'>TV Shows</h1>
        <Button variant='outline' size='sm' className='gap-2'>
          <Filter className='h-4 w-4' /> Filter
        </Button>
      </div>

      {/* Featured TV Show */}
      <div className='animate-fade-in relative mb-12 h-[50vh] w-full overflow-hidden rounded-xl'>
        <img
          src={featuredShow.image || '/placeholder.svg'}
          alt={featuredShow.title}
          className='absolute inset-0 h-full w-full object-cover'
        />
        <div className='absolute inset-0 bg-gradient-to-t from-background via-background/80 to-transparent' />

        <div className='absolute bottom-0 left-0 right-0 p-6 md:p-8'>
          <div className='flex max-w-3xl flex-col gap-4'>
            <Badge
              variant='secondary'
              className='w-fit bg-purple-500/20 text-purple-500'
            >
              <Tv className='mr-1 h-3 w-3' /> Featured Series
            </Badge>

            <h2 className='text-3xl font-bold md:text-5xl'>
              {featuredShow.title}
            </h2>
            <p className='max-w-2xl text-muted-foreground'>
              {featuredShow.description}
            </p>

            <div className='flex flex-wrap items-center gap-4 text-sm text-muted-foreground'>
              <div className='flex items-center'>
                <Star className='mr-1 h-4 w-4 fill-yellow-400 text-yellow-400' />
                <span>{featuredShow.rating}/10</span>
              </div>
              <div className='flex items-center'>
                <Calendar className='mr-1 h-4 w-4' />
                <span>{featuredShow.year}</span>
              </div>
              <div className='flex items-center'>
                <Tv className='mr-1 h-4 w-4' />
                <span>
                  {featuredShow.seasons}{' '}
                  {featuredShow.seasons === 1 ? 'Season' : 'Seasons'}
                </span>
              </div>
            </div>

            <div className='mt-2 flex gap-3'>
              <Button className='gap-2' asChild>
                <Link to={`/content/${featuredShow.id}`}>
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

      {/* TV Show Categories */}
      <Tabs
        defaultValue='all'
        value={activeTab}
        onValueChange={setActiveTab}
        className='mb-6'
      >
        <TabsList>
          <TabsTrigger value='all'>All Shows</TabsTrigger>
          <TabsTrigger value='drama'>Drama</TabsTrigger>
          <TabsTrigger value='action'>Action</TabsTrigger>
          <TabsTrigger value='animation'>Animation</TabsTrigger>
          <TabsTrigger value='fantasy'>Fantasy</TabsTrigger>
        </TabsList>
      </Tabs>

      {activeTab === 'all' ? (
        // Show all categories when "all" is selected
        <div className='animate-fade-in space-y-12'>
          {tvCategories.map(category => (
            <section key={category.id}>
              <div className='mb-4 flex items-center justify-between'>
                <h2 className='text-2xl font-semibold'>{category.title}</h2>
                <Button variant='ghost' size='sm' className='gap-1' asChild>
                  <Link to={`/view-all?category=tv&title=${category.title}`}>
                    See All <ChevronRight className='h-4 w-4' />
                  </Link>
                </Button>
              </div>

              <div className='grid grid-cols-2 gap-6 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5'>
                {category.items.slice(0, 5).map(show => (
                  <TvShowCard key={show.id} show={show} />
                ))}
              </div>
            </section>
          ))}
        </div>
      ) : (
        // Show filtered TV shows when a specific category is selected
        <div className='animate-fade-in'>
          <div className='grid grid-cols-2 gap-6 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5'>
            {tvShows
              .filter(show => {
                if (activeTab === 'drama') return show.genres.includes('Drama')
                if (activeTab === 'action')
                  return (
                    show.genres.includes('Action') ||
                    show.genres.includes('Adventure')
                  )
                if (activeTab === 'animation')
                  return show.genres.includes('Animation')
                if (activeTab === 'fantasy')
                  return (
                    show.genres.includes('Fantasy') ||
                    show.genres.includes('Sci-Fi')
                  )
                return true
              })
              .map(show => (
                <TvShowCard key={show.id} show={show} />
              ))}
          </div>
        </div>
      )}
    </div>
  )
}

// TV Show Card Component
function TvShowCard({ show }: { show: any }) {
  return (
    <Card className='group overflow-hidden transition-all duration-300 hover:shadow-lg'>
      <Link to={`/content/${show.id}`}>
        <div className='relative aspect-[2/3]'>
          <img
            src={show.image || '/placeholder.svg'}
            alt={show.title}
            className='absolute inset-0 h-full w-full object-cover transition-transform duration-500 group-hover:scale-105'
          />
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
              <h3 className='line-clamp-1 font-medium'>{show.title}</h3>
              <p className='text-sm text-muted-foreground'>
                {show.year} • {show.seasons}{' '}
                {show.seasons === 1 ? 'Season' : 'Seasons'}
              </p>
            </div>
            <Badge variant='outline' className='ml-2 bg-primary/10'>
              {show.rating}
            </Badge>
          </div>
          <div className='mt-2 flex flex-wrap gap-1'>
            {show.genres.slice(0, 2).map((genre: string) => (
              <Badge key={genre} variant='secondary' className='text-xs'>
                {genre}
              </Badge>
            ))}
            {show.genres.length > 2 && (
              <Badge variant='secondary' className='text-xs'>
                +{show.genres.length - 2}
              </Badge>
            )}
          </div>
        </CardContent>
      </Link>
    </Card>
  )
}

// Loading Skeleton
function TvShowsPageSkeleton() {
  return (
    <div className='container mx-auto px-4 py-6 md:px-6'>
      <div className='mb-6 flex items-center justify-between'>
        <Skeleton className='h-10 w-40' />
        <Skeleton className='h-9 w-24' />
      </div>

      {/* Featured TV Show Skeleton */}
      <Skeleton className='mb-12 h-[50vh] w-full rounded-xl' />

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
