import { useEffect } from 'react'
import { useTranslation } from 'react-i18next'
import { Route, Routes } from 'react-router-dom'

import { StickyNavbar } from '~/components/Navbar/Navbar'
import { VideoPlayerEventType } from '~/contexts/EventBusContext/types'
import { useEventBus } from '~/hooks/useEventBus'
import { useNotification } from '~/hooks/useNotification'
import { usePost } from '~/hooks/usePost'
import { Favorites } from '~/pages/Favorites/Favorites'
import { Library } from '~/pages/Library/Library'
import { LibraryDetail } from '~/pages/LibraryDetail/LibraryDetail'
import { MediaDetail } from '~/pages/MediaDetail/MediaDetail'
import { Settings } from '~/pages/Settings/Settings'
import { Test } from '~/pages/Test/Test'
import { Video } from '~/pages/Video/VIdeo'

export const Home = () => {
  const { addNotification } = useNotification()
  const { onEvent } = useEventBus()
  const { t } = useTranslation()
  const post = usePost()

  useEffect(() => {
    onEvent('LibrarySaved', payload =>
      addNotification({
        title: t('notification.librarySaved.title'),
        message: t('notification.librarySaved.message', {
          libraryName: payload.libraryName,
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
