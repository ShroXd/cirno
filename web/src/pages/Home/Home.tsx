import { Settings } from '../Settings/Settings'
import { Sidebar } from '../../components/Sidebar/Sidebar'
import { Library } from '../Library/Library'
import { Route, Routes } from 'react-router-dom'
import { Test } from '../Test/Test'
import { Detail } from '../Detail/Detail'

export const Home = () => {
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
          </Routes>
        </div>
      </div>
    </>
  )
}
