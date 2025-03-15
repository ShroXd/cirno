import { ReactElement } from 'react'
import { HashRouter } from 'react-router-dom'

import { render } from '@testing-library/react'

import ErrorBoundary from '~/components/ErrorBoundary/ErrorBoundary'
import { AxiosProvider } from '~/contexts/AxiosContext/AxiosContext'
import { EventBusProvider } from '~/contexts/EventBusContext/EventBusContext'
import { FeatureProvider } from '~/contexts/FeatureContext/FeatureContext'
import { NotificationProvider } from '~/contexts/NotificationContext/NotificationContext'
import { WebSocketProvider } from '~/contexts/WebSocketContext/WebSocketContext'

export const renderWithContext = (children: ReactElement) =>
  render(
    <ErrorBoundary>
      <NotificationProvider>
        <EventBusProvider>
          <WebSocketProvider>
            <AxiosProvider>
              <HashRouter>
                <FeatureProvider>{children}</FeatureProvider>
              </HashRouter>
            </AxiosProvider>
          </WebSocketProvider>
        </EventBusProvider>
      </NotificationProvider>
    </ErrorBoundary>
  )
