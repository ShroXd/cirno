import { useCallback } from 'react'
import { NavLink, useParams } from 'react-router-dom'
import { Typography } from '@material-tailwind/react'

import { MediaItemDto } from '~/bindings/MediaItemDto'
import { useFetch } from '~/hooks/useFetch'
import { wrapInGrid } from '~/pages/utils'
import { LibraryDto } from '~/bindings/LibraryDto'
import { Container } from '~/components/Container/Container'
import { ContentCard } from '~/components/ContentCard/ContentCard'

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
            <NavLink
              to={`/library/${libraryId}/media/${item.id}`}
              key={item.id.toString()}
            >
              <ContentCard
                imageUrl={item.poster_path ?? ''}
                title={item.title}
              />
            </NavLink>
          ))
        )}
      </Container>
    </>
  )
}
