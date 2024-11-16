import { HashRouter } from 'react-router-dom'
import { Home } from './pages/Home/Home'
import { WebSocketProvider } from './contexts/WebSocketContext.tsx'
import { AxiosProvider } from './contexts/AxiosContext.tsx'
import { EventBusProvider } from './contexts/EventBusContext/EventBusContext.tsx'

function App() {
  return (
    <EventBusProvider>
      <WebSocketProvider>
        <AxiosProvider>
          <HashRouter>
            <Home />
          </HashRouter>
        </AxiosProvider>
      </WebSocketProvider>
    </EventBusProvider>
  )
}

export default App
