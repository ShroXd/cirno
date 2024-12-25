import { useCallback } from 'react'
import { NavLink, useParams } from 'react-router-dom'
import { Typography } from '@material-tailwind/react'

import { MediaItemDto } from '@/bindings/MediaItemDto'
import { useFetch } from '@/hooks/useFetch'
import { wrapInGrid } from '@/pages/utils'
import { LibraryDto } from '@/bindings/LibraryDto'
import { Container } from '@/components/Container/Container'

export const LibraryDetail = () => {
  const { libraryId } = useParams()

  const {
    data: detail,
    error: detailError,
    isLoading: detailIsLoading,
  } = useFetch<LibraryDto>(`/library/${libraryId}`)

  const { data, error, isLoading } = useFetch<MediaItemDto[]>(
    `/library/${libraryId}/media`
  )
  const container = useCallback(wrapInGrid, [])

  if (isLoading || detailIsLoading) return <div>Loading...</div>
  if (error || detailError) return <div>Error: {error?.message}</div>

  return (
    <>
      <Container>
        <Typography className='mb-4 mt-2' variant='h4' color='blue-gray'>
          {detail?.name}
        </Typography>
        {container(
          data?.map(item => (
            <div
              className='max-w-sm rounded-xl cursor-pointer overflow-hidden shadow-lg hover:shadow-xl transition-shadow duration-300'
              key={item.title}
            >
              <NavLink to={`/library/${libraryId}/media/${item.id}`}>
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
      </Container>
    </>
  )
}
