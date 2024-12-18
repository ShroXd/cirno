import { useCallback, useEffect } from 'react'
import { Typography } from '@material-tailwind/react'
import { NavLink } from 'react-router-dom'
import { mutate } from 'swr'
import { useTranslation } from 'react-i18next'

import { useFetch } from '@/hooks/useFetch'
import { useEventBus } from '@/hooks/useEventBus'
import { wrapInGrid } from '@/pages/utils'
import { MediaLibraryDto } from '@/bindings/MediaLibraryDTO'

export const Library = () => {
  // TODO: fetch media libraries instead of media items
  const { data, error, isLoading } =
    useFetch<MediaLibraryDto[]>('/media-libraries/')
  const { t } = useTranslation()
  const { onEvent } = useEventBus()
  const container = useCallback(wrapInGrid, [])

  useEffect(() => {
    onEvent('MediaLibrarySaved', () => {
      mutate('/media-libraries/media-items')
    })
  }, [onEvent])

  if (isLoading) {
    const skeletons = Array.from({ length: 10 }, (_, i) => (
      <div className='skeleton h-72 w-48 animate-pulse rounded-lg' key={i} />
    ))
    return container(skeletons)
  }
  if (error) return container(<div>Error: {error.message}</div>)

  return (
    <>
      <Typography className='mb-4 mt-2' variant='h4' color='blue-gray'>
        {t('page.library.recent_added')}
      </Typography>
      {container(
        data?.map((mediaLibrary: MediaLibraryDto) => (
          <div
            className='group flex cursor-pointer select-none flex-col pb-2'
            key={mediaLibrary.id.toString()}
          >
            <NavLink to={`/library-detail/${mediaLibrary.id}`}>
              <div className='max-w-sm rounded-xl overflow-hidden shadow-lg hover:shadow-xl transition-shadow duration-300'>
                <img
                  className='w-full h-64 object-cover'
                  src={mediaLibrary.posters[0].poster_path ?? ''}
                  alt={mediaLibrary.name}
                />
                <div className='px-4 py-3 bg-white'>
                  <Typography
                    variant='paragraph'
                    className='text-center font-medium truncate'
                  >
                    {mediaLibrary.name}
                  </Typography>
                </div>
              </div>
            </NavLink>
          </div>
        ))
      )}
    </>
  )
}
