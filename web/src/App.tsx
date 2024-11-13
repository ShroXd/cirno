import { HashRouter } from 'react-router-dom'
import { Home } from './pages/Home/Home'
import { WebSocketProvider } from './contexts/WebSocketContext.tsx'
import { AxiosProvider } from './contexts/AxiosContext.tsx'

function App() {
  return (
    <WebSocketProvider>
      <AxiosProvider>
        <HashRouter>
          <Home />
        </HashRouter>
      </AxiosProvider>
    </WebSocketProvider>
  )
}

export default App
