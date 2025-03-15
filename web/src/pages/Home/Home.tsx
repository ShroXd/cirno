import { useCallback, useEffect } from 'react'
import { useTranslation } from 'react-i18next'
import { Route, Routes } from 'react-router-dom'

import { StickyNavbar } from '~/components/Navbar/Navbar'
import { Variation } from '~/components/NotificationItem/constants'
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
  const { onEvent, offEvent } = useEventBus()
  const { t } = useTranslation()
  const post = usePost()

  const handleLibrarySaved = useCallback(
    (payload: { libraryName: string }) => {
      addNotification(
        {
          title: t('notification.librarySaved.title'),
          message: t('notification.librarySaved.message', {
            libraryName: payload.libraryName,
          }),
        },
        Variation.Success
      )
    },
    [t, addNotification]
  )

  const handleError = useCallback(
    (payload: { title: string; message: string }) => {
      addNotification(payload, Variation.Error)
    },
    [addNotification]
  )

  const handleStop = useCallback(() => {
    post('/video-player/stop')
  }, [post])

  useEffect(() => {
    onEvent('LibrarySaved', handleLibrarySaved)
    onEvent('Error', handleError)
    onEvent(VideoPlayerEventType.Stop, handleStop)

    return () => {
      offEvent('LibrarySaved', handleLibrarySaved)
      offEvent('Error', handleError)
      offEvent(VideoPlayerEventType.Stop, handleStop)
    }
  }, [onEvent, offEvent, handleLibrarySaved, handleError, handleStop])

  return (
    <>
      <div className='flex h-screen flex-col'>
        <StickyNavbar />

        <div className='h-full w-auto pt-20'>
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
