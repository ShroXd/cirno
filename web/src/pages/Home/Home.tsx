import { useEffect } from 'react'
import { Route, Routes } from 'react-router-dom'
import { Settings } from '../Settings/Settings'
import { Sidebar } from '../../components/Sidebar/Sidebar'
import { Library } from '../Library/Library'
import { Test } from '../Test/Test'
import { Detail } from '../Detail/Detail'
import { Favorites } from '../Favorites/Favorites'
import { Video } from '../Video/VIdeo'
import { useNotification } from '../../hooks/useNotification'
import { useEventBus } from '../../hooks/useEventBus'
import { useTranslation } from 'react-i18next'
import { Button } from '@material-tailwind/react'

export const Home = () => {
  const { addNotification } = useNotification()
  const { onEvent } = useEventBus()
  const { t } = useTranslation()

  useEffect(() => {
    onEvent('MediaLibrarySaved', (payload: unknown) =>
      addNotification({
        message: t('notification.mediaLibrarySaved', {
          mediaLibraryName: (payload as Record<string, unknown>)[
            'media_library_name'
          ],
        }),
      })
    )
  }, [onEvent, addNotification, t])

  const testNotification = () => {
    addNotification({
      title: 'test title',
      message:
        'This is a longer test message that demonstrates the notification system. It shows how notifications can display multiple lines of text and handle longer content gracefully.',
    })
  }

  return (
    <>
      <div className='flex h-screen w-screen px-1 py-1'>
        <Sidebar />

        <div className='ml-6 h-full w-auto'>
          <Button onClick={testNotification}>test</Button>
          <Routes>
            <Route path='/' element={<Library />} />
            <Route path='/settings' element={<Settings />} />
            <Route path='/test' element={<Test />} />
            <Route path='/detail/:id' element={<Detail />} />
            <Route path='/favorites' element={<Favorites />} />
            <Route path='/video' element={<Video />} />
          </Routes>
        </div>
      </div>
    </>
  )
}
