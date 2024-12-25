import { useEffect } from 'react'
import { Route, Routes } from 'react-router-dom'
import { Settings } from '@/pages/Settings/Settings'
import { Library } from '@/pages/Library/Library'
import { Test } from '@/pages/Test/Test'
import { MediaDetail } from '@/pages/MediaDetail/MediaDetail'
import { Favorites } from '@/pages/Favorites/Favorites'
import { Video } from '@/pages/Video/VIdeo'
import { useNotification } from '@/hooks/useNotification'
import { useEventBus } from '@/hooks/useEventBus'
import { useTranslation } from 'react-i18next'
import { LibraryDetail } from '../LibraryDetail/LibraryDetail'
import { usePost } from '@/hooks/usePost'
import { VideoPlayerEventType } from '@/contexts/EventBusContext/eventBus'
import { StickyNavbar } from '@/components/Navbar/Navbar'

export const Home = () => {
  const { addNotification } = useNotification()
  const { onEvent } = useEventBus()
  const { t } = useTranslation()
  const post = usePost()

  useEffect(() => {
    onEvent('LibrarySaved', (payload: unknown) =>
      addNotification({
        title: t('notification.librarySaved.title'),
        message: t('notification.librarySaved.message', {
          libraryName: (payload as Record<string, unknown>)['library_name'],
        }),
      })
    )

    onEvent(VideoPlayerEventType.Stop, () => {
      post('/video-player/stop')
    })
  }, [onEvent, addNotification, t, post])

  return (
    <>
      <div className='flex h-screen flex-col'>
        <StickyNavbar />

        <div className='pt-20 h-full w-auto overflow-y-auto'>
          <div className='h-2'></div>
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
            <Route
              path='/library/:libraryId/media/:mediaId/video/:episodeId'
              element={<Video />}
            />
          </Routes>
        </div>
      </div>
    </>
  )
}
