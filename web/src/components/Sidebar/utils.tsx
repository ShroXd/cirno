import {
  FaceSmileIcon,
  FilmIcon,
  InboxStackIcon,
  TvIcon,
} from '@heroicons/react/24/outline'
import { ReactNode } from 'react'

import { LibraryCategory } from '@/bindings/LibraryCategory'

export const getIconAccordingToCategory = (
  category: LibraryCategory
): ReactNode => {
  switch (category) {
    case 'Movie':
      return <FilmIcon className='h-5 w-5' />
    case 'TvShow':
      return <TvIcon className='h-5 w-5' />
    case 'Animation':
      return <FaceSmileIcon className='h-5 w-5' />
    default:
      return <InboxStackIcon className='h-5 w-5' />
  }
}
