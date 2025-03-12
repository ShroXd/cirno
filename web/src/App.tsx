import { HashRouter, Route, Routes } from 'react-router-dom'

import ErrorBoundary from './components/ErrorBoundary/ErrorBoundary'
import { FeatureProvider } from './contexts/FeatureContext/FeatureContext'
import { AxiosProvider } from '~/contexts/AxiosContext/AxiosContext.tsx'
import { EventBusProvider } from '~/contexts/EventBusContext/EventBusContext.tsx'
import { NotificationProvider } from '~/contexts/NotificationContext/NotificationContext.tsx'
import { WebSocketProvider } from '~/contexts/WebSocketContext/WebSocketContext.tsx'
import Layout from './layout'
import { SidebarProvider } from './components/ui/sidebar'
import Home from './pages/Home'
import ContentDetailPage from './pages/ContentDetail'
import PlaylistsPage from './pages/PlaylistsPage'
import DiscoverPage from './pages/DiscoverPage'
import SearchPage from './pages/SearchPage'
import LibraryPage from './pages/LibraryPage'
import MoviesPage from './pages/MoviesPage'
import TvShowsPage from './pages/TvShowsPage'
import FavoritesPage from './pages/FavoritesPage'
import SettingsPage from './pages/SettingsPage'
import WatchPage from './pages/WatchPage'

function App() {
  return (
    <ErrorBoundary>
      <NotificationProvider>
        <EventBusProvider>
          <WebSocketProvider>
            <AxiosProvider>
              <HashRouter>
                <FeatureProvider>
                  <SidebarProvider>
                    <Routes>
                      <Route path="/" element={<Layout />}>
                        <Route index element={<Home />} />
                        <Route path="content/:id" element={<ContentDetailPage />} />
                        <Route path="playlists" element={<PlaylistsPage />} />
                        <Route path="discover" element={<DiscoverPage />} />
                        <Route path="search" element={<SearchPage />} />
                        <Route path="library" element={<LibraryPage />} />
                        <Route path="movies" element={<MoviesPage />} />
                        <Route path="tv-shows" element={<TvShowsPage />} />
                        <Route path="favorites" element={<FavoritesPage />} />
                        <Route path="settings" element={<SettingsPage />} />
                      </Route>
                      <Route path="watch/:id" element={<WatchPage />} />
                    </Routes>
                  </SidebarProvider>
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
