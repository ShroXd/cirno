import { Film, Home, Library, PlaySquare, Star } from 'lucide-react'

export const menuItems = [
  {
    label: 'Home',
    icon: <Home className='mr-2 h-4 w-4' />,
    path: '/',
  },
  {
    label: 'Library',
    icon: <Library className='mr-2 h-4 w-4' />,
    path: '/library',
  },
  // {
  //   label: 'Discover',
  //   icon: <Compass className='mr-2 h-4 w-4' />,
  //   path: '/discover',
  // },
  // {
  //   label: 'Search',
  //   icon: <Search className='mr-2 h-4 w-4' />,
  //   path: '/search',
  // },
]

export const libraryItems = [
  {
    label: 'All Content',
    icon: <Library className='mr-2 h-4 w-4' />,
    path: '/library',
  },
  {
    label: 'Movies',
    icon: <Film className='mr-2 h-4 w-4' />,
    path: '/movies',
  },
  {
    label: 'TV Shows',
    icon: <PlaySquare className='mr-2 h-4 w-4' />,
    path: '/tv-shows',
  },
  {
    label: 'Favorites',
    icon: <Star className='mr-2 h-4 w-4' />,
    path: '/favorites',
  },
]
