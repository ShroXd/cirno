import { HashRouter } from 'react-router-dom'

import { Home } from '~/pages/Home/Home'
import { WebSocketProvider } from '~/contexts/WebSocketContext/WebSocketContext.tsx'
import { AxiosProvider } from '~/contexts/AxiosContext/AxiosContext.tsx'
import { EventBusProvider } from '~/contexts/EventBusContext/EventBusContext.tsx'
import { NotificationProvider } from '~/contexts/NotificationContext.tsx'

function App() {
  return (
    <NotificationProvider>
      <EventBusProvider>
        <WebSocketProvider>
          <AxiosProvider>
            <HashRouter>
              <Home />
            </HashRouter>
          </AxiosProvider>
        </WebSocketProvider>
      </EventBusProvider>
    </NotificationProvider>
  )
}

export default App
