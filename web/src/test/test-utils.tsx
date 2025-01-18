import { ReactElement } from 'react'
import { HashRouter } from 'react-router-dom'

import { ThemeProvider } from '@material-tailwind/react'
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
                <ThemeProvider>
                  <FeatureProvider>{children}</FeatureProvider>
                </ThemeProvider>
              </HashRouter>
            </AxiosProvider>
          </WebSocketProvider>
        </EventBusProvider>
      </NotificationProvider>
    </ErrorBoundary>
  )
