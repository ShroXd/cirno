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
  const { data, error, isLoading } = useFetch<LibraryDto[]>('/library/')
  const { onEvent } = useEventBus()
  const container = useCallback(wrapInGrid, [])
  const { t } = useTranslation()

  useEffect(() => {
    onEvent('LibrarySaved', () => {
      mutate('/library/')
    })
  }, [onEvent])

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
            data?.map((library: LibraryDto) => (
              <NavLink
                to={`/library/${library.id}`}
                key={library.id.toString()}
              >
                <ContentCard
                  imageUrl={library.posters?.[0]?.poster_path ?? ''}
                  title={library.name}
                />
              </NavLink>
            ))
          )}
        </AsyncSwitcher>
      </Container>
    </>
  )
}
