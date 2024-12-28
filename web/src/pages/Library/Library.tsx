import { useCallback, useEffect } from 'react'
import { Typography } from '@material-tailwind/react'
import { NavLink } from 'react-router-dom'
import { mutate } from 'swr'

import { useFetch } from '~/hooks/useFetch'
import { useEventBus } from '~/hooks/useEventBus'
import { wrapInGrid } from '~/pages/utils'
import { LibraryDto } from '~/bindings/LibraryDto'
import { Container } from '~/components/Container/Container'
import { useTranslation } from 'react-i18next'
import { ContentCard } from '~/components/ContentCard/ContentCard'

export const Library = () => {
  // TODO: fetch media libraries instead of media items
  const { data, error, isLoading } = useFetch<LibraryDto[]>('/library/')
  const { onEvent } = useEventBus()
  const container = useCallback(wrapInGrid, [])
  const { t } = useTranslation()

  useEffect(() => {
    onEvent('LibrarySaved', () => {
      mutate('/library/')
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
      <Container>
        <Typography className='mb-4 mt-2' variant='h4' color='blue-gray'>
          {t('page.library.recent_added')}
        </Typography>
        {container(
          data?.map((library: LibraryDto) => (
            <NavLink to={`/library/${library.id}`} key={library.id.toString()}>
              <ContentCard
                imageUrl={library.posters[0].poster_path ?? ''}
                title={library.name}
              />
            </NavLink>
          ))
        )}
      </Container>
    </>
  )
}
