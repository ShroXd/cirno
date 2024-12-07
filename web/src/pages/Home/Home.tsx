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

export const Home = () => {
  const { addNotification } = useNotification()
  const { listenForMessages } = useEventBus()

  useEffect(() => {
    listenForMessages('MediaLibraryScanned', (payload: unknown) => {
      addNotification({
        message: `Media library scanned: ${(payload as Array<unknown>)[1]}`,
      })
    })
  }, [listenForMessages, addNotification])

  return (
    <>
      <div className='flex h-screen w-screen px-1 py-1'>
        <Sidebar />

        <div className='ml-6 h-full w-auto'>
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
