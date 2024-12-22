import { useEffect } from 'react'
import { Route, Routes, useLocation } from 'react-router-dom'
import { Settings } from '@/pages/Settings/Settings'
import { Sidebar } from '@/components/Sidebar/Sidebar'
import { Library } from '@/pages/Library/Library'
import { Test } from '@/pages/Test/Test'
import { MediaDetail } from '@/pages/MediaDetail/MediaDetail'
import { Favorites } from '@/pages/Favorites/Favorites'
import { Video } from '@/pages/Video/VIdeo'
import { useNotification } from '@/hooks/useNotification'
import { useEventBus } from '@/hooks/useEventBus'
import { useTranslation } from 'react-i18next'
import { LibraryDetail } from '../LibraryDetail/LibraryDetail'
import Breadcrumb from '@/components/Breadcrumb/Breadcrumb'
import { Container } from '@/components/Container/Container'

export const Home = () => {
  const { addNotification } = useNotification()
  const { onEvent } = useEventBus()
  const { t } = useTranslation()
  const location = useLocation()

  useEffect(() => {
    onEvent('LibrarySaved', (payload: unknown) =>
      addNotification({
        title: t('notification.librarySaved.title'),
        message: t('notification.librarySaved.message', {
          libraryName: (payload as Record<string, unknown>)['library_name'],
        }),
      })
    )
  }, [onEvent, addNotification, t])

  return (
    <>
      <div className='flex h-screen w-screen px-1 py-1'>
        <Sidebar />

        <div className='mt-2 h-full w-auto'>
          {location.pathname !== '/' && (
            <Container className='mb-6'>
              <Breadcrumb />
            </Container>
          )}
          <Routes>
            <Route path='/' element={<Library />} />
            <Route path='/library/:libraryId' element={<LibraryDetail />} />
            <Route path='/settings' element={<Settings />} />
            <Route path='/test' element={<Test />} />
            <Route
              path='/library/:libraryId/media/:mediaId'
              element={<MediaDetail />}
            />
            <Route path='/favorites' element={<Favorites />} />
            <Route path='/video' element={<Video />} />
          </Routes>
        </div>
      </div>
    </>
  )
}
