import { HashRouter } from 'react-router-dom'

import ErrorBoundary from './components/ErrorBoundary/ErrorBoundary'
import { FeatureProvider } from './contexts/FeatureContext/FeatureContext'
import { AxiosProvider } from '~/contexts/AxiosContext/AxiosContext.tsx'
import { EventBusProvider } from '~/contexts/EventBusContext/EventBusContext.tsx'
import { NotificationProvider } from '~/contexts/NotificationContext/NotificationContext.tsx'
import { WebSocketProvider } from '~/contexts/WebSocketContext/WebSocketContext.tsx'
import { Home } from '~/pages/Home/Home'

function App() {
  return (
    <ErrorBoundary>
      <NotificationProvider>
        <EventBusProvider>
          <WebSocketProvider>
            <AxiosProvider>
              <HashRouter>
                <FeatureProvider>
                  <Home />
                </FeatureProvider>
              </HashRouter>
            </AxiosProvider>
          </WebSocketProvider>
        </EventBusProvider>
      </NotificationProvider>
    </ErrorBoundary>
  )
}

export default App
