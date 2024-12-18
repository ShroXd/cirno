import { useEffect } from 'react'
import { Route, Routes } from 'react-router-dom'
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

export const Home = () => {
  const { addNotification } = useNotification()
  const { onEvent } = useEventBus()
  const { t } = useTranslation()

  useEffect(() => {
    onEvent('MediaLibrarySaved', (payload: unknown) =>
      addNotification({
        title: t('notification.mediaLibrarySaved.title'),
        message: t('notification.mediaLibrarySaved.message', {
          mediaLibraryName: (payload as Record<string, unknown>)[
            'media_library_name'
          ],
        }),
      })
    )
  }, [onEvent, addNotification, t])

  return (
    <>
      <div className='flex h-screen w-screen px-1 py-1'>
        <Sidebar />

        <div className='ml-6 h-full w-auto'>
          <Routes>
            <Route path='/' element={<Library />} />
            <Route path='/library-detail/:id' element={<LibraryDetail />} />
            <Route path='/settings' element={<Settings />} />
            <Route path='/test' element={<Test />} />
            <Route path='/media-detail/:id' element={<MediaDetail />} />
            <Route path='/favorites' element={<Favorites />} />
            <Route path='/video' element={<Video />} />
          </Routes>
        </div>
      </div>
    </>
  )
}
