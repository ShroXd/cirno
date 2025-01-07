import { useCallback, useEffect, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { NavLink } from 'react-router-dom'

import { Button, Typography } from '@material-tailwind/react'
import { mutate } from 'swr'

import { LibraryDto } from '~/bindings/LibraryDto'
import { AsyncSwitcher } from '~/components/AsyncSwitcher/AsyncSwitcher'
import { Container } from '~/components/Container/Container'
import { ContentCard } from '~/components/ContentCard/ContentCard'
import { ContentCardSkeleton } from '~/components/ContentCard/ContentCardSkeleton'
import { FeatureToggle } from '~/components/FeatureToggle/FeatureToggle'
import { useEventBus } from '~/hooks/useEventBus'
import { useFetch } from '~/hooks/useFetch'
import { wrapInGrid } from '~/pages/utils'

export const Library = () => {
  const [hasError, setHasError] = useState(false)
  const [scanningLibraryIds, setScanningLibraryIds] = useState<Set<number>>(
    new Set()
  )
  const { data, error, isLoading } = useFetch<LibraryDto[]>('/library/')
  const { onEvent, offEvent } = useEventBus()
  const container = useCallback(wrapInGrid, [])
  const { t } = useTranslation()

  useEffect(() => {
    const handleScanning = (payload: { libraryId: number }) => {
      setScanningLibraryIds(prev => new Set([...prev, payload.libraryId]))
    }

    const handleSaved = (payload: { libraryId: number }) => {
      console.log('Handling save event for library:', payload.libraryId)
      setScanningLibraryIds(prev => {
        const newIds = [...prev].filter(id => id !== payload.libraryId)
        console.log('Updated scanning ids:', newIds)
        const newSet = new Set(newIds)
        console.log('Updated scanning ids:', [...newSet])
        return newSet
      })

      mutate('/library/', undefined, { revalidate: true })
    }

    onEvent('LibraryScanning', handleScanning)
    onEvent('LibrarySaved', handleSaved)

    return () => {
      offEvent('LibraryScanning', handleScanning)
      offEvent('LibrarySaved', handleSaved)
    }
  }, [offEvent, onEvent])

  if (hasError) {
    throw new Error('test')
  }

  return (
    <>
      <Container>
        <FeatureToggle featureId='errorButton'>
          <Button onClick={() => setHasError(true)}>Trigger Error</Button>
        </FeatureToggle>
        <Typography className='mb-4 mt-2' variant='h4' color='blue-gray'>
          {t('page.library.recent_added')}
        </Typography>
        <AsyncSwitcher
          loading={isLoading}
          error={error}
          data={data}
          loadingComponent={container(
            <>
              <ContentCardSkeleton />
              <ContentCardSkeleton />
            </>
          )}
        >
          {container(
            <>
              {data?.map((library: LibraryDto) => (
                <NavLink
                  to={`/library/${library.id}`}
                  key={library.id.toString()}
                >
                  <ContentCard
                    imageUrl={library.posters?.[0]?.poster_path ?? ''}
                    title={library.name}
                  />
                </NavLink>
              ))}
              {Array.from(scanningLibraryIds).map(id => (
                <ContentCardSkeleton key={id} />
              ))}
            </>
          )}
        </AsyncSwitcher>
      </Container>
    </>
  )
}
