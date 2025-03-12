import { useEffect, useState } from 'react'
import { Link } from 'react-router-dom'

import {
  ChevronDown,
  Film,
  Play,
  Search,
  SlidersHorizontal,
  Star,
  Tv,
} from 'lucide-react'

import { Badge } from '../components/ui/badge'
import { Button } from '../components/ui/button'
import { Card, CardContent } from '../components/ui/card'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '../components/ui/dropdown-menu'
import { Input } from '../components/ui/input'
import { Skeleton } from '../components/ui/skeleton'
import { Tabs, TabsList, TabsTrigger } from '../components/ui/tabs'

// Mock library content
const libraryContent = [
  {
    id: '1',
    title: 'Interstellar',
    description:
      "A team of explorers travel through a wormhole in space in an attempt to ensure humanity's survival.",
    image: '/placeholder.svg?height=300&width=200',
    type: 'movie',
    year: 2014,
    rating: 8.7,
    genres: ['Sci-Fi', 'Adventure', 'Drama'],
    duration: '2h 49m',
    added: '2023-12-15',
  },
  {
    id: '2',
    title: 'Breaking Bad',
    description:
      'A high school chemistry teacher diagnosed with inoperable lung cancer turns to manufacturing and selling methamphetamine.',
    image: '/placeholder.svg?height=300&width=200',
    type: 'tv',
    year: '2008-2013',
    seasons: 5,
    episodes: 62,
    rating: 9.5,
    genres: ['Crime', 'Drama', 'Thriller'],
    added: '2023-11-20',
  },
  {
    id: '3',
    title: 'The Shawshank Redemption',
    description:
      'Two imprisoned men bond over a number of years, finding solace and eventual redemption through acts of common decency.',
    image: '/placeholder.svg?height=300&width=200',
    type: 'movie',
    year: 1994,
    rating: 9.3,
    genres: ['Drama'],
    duration: '2h 22m',
    added: '2024-01-05',
  },
  {
    id: '4',
    title: 'Game of Thrones',
    description:
      'Nine noble families fight for control over the lands of Westeros, while an ancient enemy returns.',
    image: '/placeholder.svg?height=300&width=200',
    type: 'tv',
    year: '2011-2019',
    seasons: 8,
    episodes: 73,
    rating: 9.2,
    genres: ['Action', 'Adventure', 'Drama'],
    added: '2023-10-12',
  },
  {
    id: '5',
    title: 'Dune',
    description:
      "Feature adaptation of Frank Herbert's science fiction novel about the son of a noble family entrusted with the protection of the most valuable asset in the galaxy.",
    image: '/placeholder.svg?height=300&width=200',
    type: 'movie',
    year: 2021,
    rating: 8.0,
    genres: ['Sci-Fi', 'Adventure'],
    duration: '2h 35m',
    added: '2024-02-18',
  },
  {
    id: '6',
    title: 'Succession',
    description:
      'The Roy family is known for controlling the biggest media and entertainment company in the world.',
    image: '/placeholder.svg?height=300&width=200',
    type: 'tv',
    year: '2018-2023',
    seasons: 4,
    episodes: 40,
    rating: 8.8,
    genres: ['Drama'],
    added: '2023-09-30',
  },
  {
    id: '7',
    title: 'The Batman',
    description:
      'When a sadistic serial killer begins murdering key political figures in Gotham, Batman is forced to investigate.',
    image: '/placeholder.svg?height=300&width=200',
    type: 'movie',
    year: 2022,
    rating: 7.8,
    genres: ['Action', 'Crime', 'Drama'],
    duration: '2h 56m',
    added: '2024-01-22',
  },
  {
    id: '8',
    title: 'Stranger Things',
    description:
      'When a young boy disappears, his mother, a police chief, and his friends must confront terrifying supernatural forces.',
    image: '/placeholder.svg?height=300&width=200',
    type: 'tv',
    year: '2016-Present',
    seasons: 4,
    episodes: 34,
    rating: 8.7,
    genres: ['Drama', 'Fantasy', 'Horror'],
    added: '2023-12-05',
  },
  {
    id: '9',
    title: 'Soul',
    description:
      'A musician who has lost his passion for music is transported out of his body and must find his way back with the help of an infant soul learning about herself.',
    image: '/placeholder.svg?height=300&width=200',
    type: 'movie',
    year: 2020,
    rating: 8.0,
    genres: ['Animation', 'Adventure', 'Comedy'],
    duration: '1h 40m',
    added: '2024-03-01',
  },
  {
    id: '10',
    title: 'Arcane',
    description:
      'Set in utopian Piltover and the oppressed underground of Zaun, the story follows the origins of two iconic League champions-and the power that will tear them apart.',
    image: '/placeholder.svg?height=300&width=200',
    type: 'tv',
    year: '2021-Present',
    seasons: 1,
    episodes: 9,
    rating: 9.0,
    genres: ['Animation', 'Action', 'Adventure'],
    added: '2023-11-15',
  },
  {
    id: '11',
    title: 'Spider-Man: Into the Spider-Verse',
    description:
      'Teen Miles Morales becomes the Spider-Man of his universe, and must join with five spider-powered individuals from other dimensions to stop a threat for all realities.',
    image: '/placeholder.svg?height=300&width=200',
    type: 'movie',
    year: 2018,
    rating: 8.4,
    genres: ['Animation', 'Action', 'Adventure'],
    duration: '1h 57m',
    added: '2023-10-05',
  },
  {
    id: '12',
    title: 'Attack on Titan',
    description:
      'After his hometown is destroyed and his mother is killed, young Eren Jaeger vows to cleanse the earth of the giant humanoid Titans that have brought humanity to the brink of extinction.',
    image: '/placeholder.svg?height=300&width=200',
    type: 'tv',
    year: '2013-2023',
    seasons: 4,
    episodes: 88,
    rating: 9.0,
    genres: ['Animation', 'Action', 'Adventure'],
    added: '2024-02-10',
  },
]

export default function LibraryPage() {
  const [isLoading, setIsLoading] = useState(true)
  const [activeTab, setActiveTab] = useState('all')
  const [searchQuery, setSearchQuery] = useState('')
  const [sortBy, setSortBy] = useState('recently-added')
  const [viewMode, setViewMode] = useState('grid')
  const [filteredContent, setFilteredContent] = useState(libraryContent)

  // Simulate loading state
  useEffect(() => {
    const timer = setTimeout(() => {
      setIsLoading(false)
    }, 1000)

    return () => clearTimeout(timer)
  }, [])

  // Filter and sort content based on active tab, search query, and sort option
  useEffect(() => {
    let result = [...libraryContent]

    // Filter by content type
    if (activeTab === 'movies') {
      result = result.filter(item => item.type === 'movie')
    } else if (activeTab === 'tv') {
      result = result.filter(item => item.type === 'tv')
    } else if (activeTab === 'animation') {
      result = result.filter(item => item.genres.includes('Animation'))
    }

    // Filter by search query
    if (searchQuery) {
      const query = searchQuery.toLowerCase()
      result = result.filter(
        item =>
          item.title.toLowerCase().includes(query) ||
          item.description.toLowerCase().includes(query) ||
          item.genres.some(genre => genre.toLowerCase().includes(query))
      )
    }

    // Sort content
    if (sortBy === 'recently-added') {
      result.sort(
        (a, b) => new Date(b.added).getTime() - new Date(a.added).getTime()
      )
    } else if (sortBy === 'title-asc') {
      result.sort((a, b) => a.title.localeCompare(b.title))
    } else if (sortBy === 'title-desc') {
      result.sort((a, b) => b.title.localeCompare(a.title))
    } else if (sortBy === 'rating') {
      result.sort((a, b) => b.rating - a.rating)
    } else if (sortBy === 'year-desc') {
      result.sort((a, b) => {
        const yearA =
          typeof a.year === 'string'
            ? Number.parseInt(a.year.split('-')[0])
            : a.year
        const yearB =
          typeof b.year === 'string'
            ? Number.parseInt(b.year.split('-')[0])
            : b.year
        return yearB - yearA
      })
    } else if (sortBy === 'year-asc') {
      result.sort((a, b) => {
        const yearA =
          typeof a.year === 'string'
            ? Number.parseInt(a.year.split('-')[0])
            : a.year
        const yearB =
          typeof b.year === 'string'
            ? Number.parseInt(b.year.split('-')[0])
            : b.year
        return yearA - yearB
      })
    }

    setFilteredContent(result)
  }, [activeTab, searchQuery, sortBy])

  if (isLoading) {
    return <LibraryPageSkeleton />
  }

  return (
    <div className='container mx-auto px-4 py-6 md:px-6'>
      <h1 className='mb-6 text-3xl font-bold'>Library</h1>

      {/* Filters and Search */}
      <div className='mb-6 flex flex-col gap-4 md:flex-row'>
        <div className='relative flex-1'>
          <Search className='absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground' />
          <Input
            type='text'
            placeholder='Search your library...'
            className='pl-9'
            value={searchQuery}
            onChange={e => setSearchQuery(e.target.value)}
          />
        </div>

        <div className='flex gap-2'>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant='outline' className='gap-2'>
                <SlidersHorizontal className='h-4 w-4' />
                <span className='hidden sm:inline'>Sort by:</span>
                {sortBy === 'recently-added' && 'Recently Added'}
                {sortBy === 'title-asc' && 'Title (A-Z)'}
                {sortBy === 'title-desc' && 'Title (Z-A)'}
                {sortBy === 'rating' && 'Rating'}
                {sortBy === 'year-desc' && 'Newest'}
                {sortBy === 'year-asc' && 'Oldest'}
                <ChevronDown className='ml-1 h-4 w-4' />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align='end'>
              <DropdownMenuLabel>Sort by</DropdownMenuLabel>
              <DropdownMenuSeparator />
              <DropdownMenuItem onClick={() => setSortBy('recently-added')}>
                Recently Added
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => setSortBy('title-asc')}>
                Title (A-Z)
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => setSortBy('title-desc')}>
                Title (Z-A)
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => setSortBy('rating')}>
                Rating
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => setSortBy('year-desc')}>
                Newest
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => setSortBy('year-asc')}>
                Oldest
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>

          <div className='flex overflow-hidden rounded-md border'>
            <Button
              variant={viewMode === 'grid' ? 'default' : 'ghost'}
              size='sm'
              className='rounded-none'
              onClick={() => setViewMode('grid')}
            >
              <svg
                xmlns='http://www.w3.org/2000/svg'
                width='16'
                height='16'
                viewBox='0 0 24 24'
                fill='none'
                stroke='currentColor'
                strokeWidth='2'
                strokeLinecap='round'
                strokeLinejoin='round'
              >
                <rect x='3' y='3' width='7' height='7' />
                <rect x='14' y='3' width='7' height='7' />
                <rect x='3' y='14' width='7' height='7' />
                <rect x='14' y='14' width='7' height='7' />
              </svg>
            </Button>
            <Button
              variant={viewMode === 'list' ? 'default' : 'ghost'}
              size='sm'
              className='rounded-none'
              onClick={() => setViewMode('list')}
            >
              <svg
                xmlns='http://www.w3.org/2000/svg'
                width='16'
                height='16'
                viewBox='0 0 24 24'
                fill='none'
                stroke='currentColor'
                strokeWidth='2'
                strokeLinecap='round'
                strokeLinejoin='round'
              >
                <line x1='3' y1='6' x2='21' y2='6' />
                <line x1='3' y1='12' x2='21' y2='12' />
                <line x1='3' y1='18' x2='21' y2='18' />
              </svg>
            </Button>
          </div>
        </div>
      </div>

      {/* Content Tabs */}
      <Tabs
        defaultValue='all'
        value={activeTab}
        onValueChange={setActiveTab}
        className='mb-6'
      >
        <TabsList>
          <TabsTrigger value='all'>All Content</TabsTrigger>
          <TabsTrigger value='movies'>Movies</TabsTrigger>
          <TabsTrigger value='tv'>TV Shows</TabsTrigger>
          <TabsTrigger value='animation'>Animation</TabsTrigger>
        </TabsList>
      </Tabs>

      {/* Content Display */}
      {filteredContent.length === 0 ? (
        <div className='py-12 text-center'>
          <div className='mb-4 inline-flex h-16 w-16 items-center justify-center rounded-full bg-muted'>
            <Search className='h-8 w-8 text-muted-foreground' />
          </div>
          <h2 className='mb-2 text-xl font-semibold'>No results found</h2>
          <p className='mb-6 text-muted-foreground'>
            We couldn't find any content matching your search criteria.
          </p>
          <Button
            onClick={() => {
              setSearchQuery('')
              setActiveTab('all')
            }}
          >
            Clear Filters
          </Button>
        </div>
      ) : viewMode === 'grid' ? (
        <div className='animate-fade-in grid grid-cols-2 gap-6 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5'>
          {filteredContent.map(item => (
            <LibraryGridCard key={item.id} item={item} />
          ))}
        </div>
      ) : (
        <div className='animate-fade-in space-y-4'>
          {filteredContent.map(item => (
            <LibraryListCard key={item.id} item={item} />
          ))}
        </div>
      )}
    </div>
  )
}

// Library Grid Card Component
function LibraryGridCard({ item }: { item: any }) {
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
                <Play className='h-4 w-4' /> Play
              </Button>
            </div>
          </div>
        </div>
        <CardContent className='p-3'>
          <div className='flex items-start justify-between'>
            <div>
              <h3 className='line-clamp-1 font-medium'>{item.title}</h3>
              <p className='text-sm text-muted-foreground'>
                {item.type === 'movie' ? item.year : item.year}
              </p>
            </div>
            <Badge variant='outline' className='ml-2 bg-primary/10'>
              {item.rating}
            </Badge>
          </div>
          <div className='mt-2 flex flex-wrap gap-1'>
            {item.genres.slice(0, 2).map((genre: string) => (
              <Badge key={genre} variant='secondary' className='text-xs'>
                {genre}
              </Badge>
            ))}
            {item.genres.length > 2 && (
              <Badge variant='secondary' className='text-xs'>
                +{item.genres.length - 2}
              </Badge>
            )}
          </div>
        </CardContent>
      </Link>
    </Card>
  )
}

// Library List Card Component
function LibraryListCard({ item }: { item: any }) {
  return (
    <Card className='group overflow-hidden transition-all duration-300 hover:shadow-lg'>
      <Link to={`/content/${item.id}`}>
        <div className='flex flex-col sm:flex-row'>
          <div className='relative aspect-video w-full sm:aspect-[2/3] sm:w-48'>
            <img
              src={item.image || '/placeholder.svg'}
              alt={item.title}
              className='absolute inset-0 h-full w-full object-cover transition-transform duration-500 group-hover:scale-105'
            />
            <div className='absolute inset-0 flex items-center justify-center bg-gradient-to-t from-black/80 to-transparent transition-opacity duration-300 group-hover:opacity-100 sm:opacity-0'>
              <Button size='sm' variant='secondary' className='gap-2'>
                <Play className='h-4 w-4' /> Play
              </Button>
            </div>
          </div>
          <div className='flex-1 p-4'>
            <div className='mb-2 flex items-start justify-between'>
              <div>
                <h3 className='text-lg font-medium'>{item.title}</h3>
                <div className='mt-1 flex items-center gap-3 text-sm text-muted-foreground'>
                  <span>{item.type === 'movie' ? item.year : item.year}</span>
                  <span className='flex items-center'>
                    <Star className='mr-1 h-3 w-3 fill-yellow-400 text-yellow-400' />
                    {item.rating}
                  </span>
                  {item.type === 'movie' ? (
                    <span>{item.duration}</span>
                  ) : (
                    <span>
                      {item.seasons} {item.seasons === 1 ? 'Season' : 'Seasons'}
                    </span>
                  )}
                </div>
              </div>
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
            <p className='mb-3 line-clamp-2 text-sm text-muted-foreground'>
              {item.description}
            </p>
            <div className='flex flex-wrap gap-1'>
              {item.genres.map((genre: string) => (
                <Badge key={genre} variant='secondary' className='text-xs'>
                  {genre}
                </Badge>
              ))}
            </div>
          </div>
        </div>
      </Link>
    </Card>
  )
}

// Loading Skeleton
function LibraryPageSkeleton() {
  return (
    <div className='container mx-auto px-4 py-6 md:px-6'>
      <Skeleton className='mb-6 h-10 w-40' />

      <div className='mb-6 flex flex-col gap-4 md:flex-row'>
        <Skeleton className='h-10 flex-1' />
        <div className='flex gap-2'>
          <Skeleton className='h-10 w-40' />
          <Skeleton className='h-10 w-20' />
        </div>
      </div>

      <Skeleton className='mb-6 h-10 w-96' />

      <div className='grid grid-cols-2 gap-6 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5'>
        {Array.from({ length: 10 }).map((_, i) => (
          <div key={i} className='flex flex-col space-y-3'>
            <Skeleton className='aspect-[2/3] rounded-md' />
            <Skeleton className='h-5 w-full' />
            <Skeleton className='h-4 w-2/3' />
            <Skeleton className='h-4 w-1/2' />
          </div>
        ))}
      </div>
    </div>
  )
}
