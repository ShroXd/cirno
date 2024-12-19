import { MediaItemDto } from '@/bindings/MediaItemDto'
import { useFetch } from '@/hooks/useFetch'
import { useCallback } from 'react'
import { NavLink, useParams } from 'react-router-dom'
import { wrapInGrid } from '@/pages/utils'
import { Typography } from '@material-tailwind/react'
import { MediaLibraryDto } from '@/bindings/MediaLibraryDto'

export const LibraryDetail = () => {
  const { id } = useParams()
  const {
    data: detail,
    error: detailError,
    isLoading: detailIsLoading,
  } = useFetch<MediaLibraryDto>(`/library/${id}`)

  const { data, error, isLoading } = useFetch<MediaItemDto[]>(
    `/library/${id}/media`
  )

  const container = useCallback(wrapInGrid, [])

  if (isLoading || detailIsLoading) return <div>Loading...</div>
  if (error || detailError) return <div>Error: {error?.message}</div>

  return (
    <>
      <Typography className='mb-4 mt-2' variant='h4' color='blue-gray'>
        {detail?.name}
      </Typography>
      {container(
        data?.map(item => (
          <div
            className='max-w-sm rounded-xl cursor-pointer overflow-hidden shadow-lg hover:shadow-xl transition-shadow duration-300'
            key={item.title}
          >
            <NavLink to={`/media-detail/${item.id}`} state={{ detail: item }}>
              <img
                className='w-full h-64 object-cover'
                src={item.poster_path ?? ''}
                alt={item.title}
              />
              <div className='px-4 py-3 bg-white'>
                <Typography variant='paragraph'>{item.title}</Typography>
              </div>
            </NavLink>
          </div>
        ))
      )}
    </>
  )
}
