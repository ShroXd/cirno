import { useCallback, useEffect, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { Link } from 'react-router-dom'

import { mutate } from 'swr'

import { Tabs, TabsContent, TabsList, TabsTrigger } from '../components/ui/tabs'
import { MediaItemDto } from '~/bindings/MediaItemDto'
import { AnimatedSection } from '~/components/AnimatedSection/AnimatedSection'
import { AsyncSwitcher } from '~/components/AsyncSwitcher/AsyncSwitcher'
import { VerticalCard } from '~/components/VerticalCard/VerticalCard'
import { useEventBus } from '~/hooks/useEventBus'
import { useFetch } from '~/hooks/useFetch'

export default function HomePage() {
  const [_scanningLibraryIds, setScanningLibraryIds] = useState<Set<number>>(
    new Set()
  )
  const { data, error, isLoading } =
    useFetch<MediaItemDto[]>('/library/1/media')

  const { onEvent, offEvent } = useEventBus()
  const { t } = useTranslation()

  const handleScanning = useCallback((payload: { libraryId: number }) => {
    console.log('Handling scanning event for library:', payload.libraryId)
    setScanningLibraryIds(prev => new Set([...prev, payload.libraryId]))
  }, [])

  const handleSaved = useCallback((payload: { libraryId: number }) => {
    console.log('Handling saved event for library:', payload.libraryId)
    setScanningLibraryIds(prev => {
      const newIds = [...prev].filter(id => id !== payload.libraryId)
      const newSet = new Set(newIds)
      return newSet
    })

    mutate('/library/')
  }, [])

  useEffect(() => {
    mutate('/library/')
  }, [])

  useEffect(() => {
    onEvent('LibraryScanning', handleScanning)
    onEvent('LibrarySaved', handleSaved)

    return () => {
      offEvent('LibraryScanning', handleScanning)
      offEvent('LibrarySaved', handleSaved)
    }
  }, [offEvent, onEvent, handleScanning, handleSaved])

  const renderContent = () => (
    <AnimatedSection delay={0.2} className='lg:col-span-1'>
      <div className='flex w-max space-x-4 p-1'>
        {data?.map(media => (
          <Link to={`/content/${media.id}`} key={media.title}>
            <VerticalCard
              title={media.title}
              posterPath={media.poster_path || ''}
              plot={media.plot || ''}
              category={'movie'}
              year={media?.year?.toString() || '2020'}
              duration={'2h'}
              episodeCount={1}
            />
          </Link>
        ))}
      </div>
    </AnimatedSection>
  )

  return (
    <div className='ml-6 mr-4 h-screen overflow-y-auto px-4 py-6 md:px-6'>
      <h1 className='mb-6 text-3xl font-bold'>{t('page.home.welcome')}</h1>

      <Tabs defaultValue='all' className='mb-8'>
        <TabsList>
          <TabsTrigger value='all'>{t('page.home.tab.all')}</TabsTrigger>
          <TabsTrigger value='favorites'>
            {t('page.home.tab.anime')}
          </TabsTrigger>
          <TabsTrigger value='movies'>{t('page.home.tab.movies')}</TabsTrigger>
          <TabsTrigger value='tv'>{t('page.home.tab.tv')}</TabsTrigger>
        </TabsList>

        <TabsContent value='all' className='mt-6'>
          <section className='mb-10'>
            <div className='mb-4 flex items-center justify-between'>
              <h2 className='text-2xl font-semibold'>
                {t('page.home.recent_added')}
              </h2>
            </div>
            <AsyncSwitcher
              loading={isLoading}
              error={error}
              data={data}
              loadingComponent={renderContent()}
            >
              {renderContent()}
            </AsyncSwitcher>
          </section>
        </TabsContent>
      </Tabs>
    </div>
  )
}
