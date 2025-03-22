import { Feature } from '~/contexts/FeatureContext/FeatureContext'

export const defaultFeatures: Feature[] = [
  {
    id: 'edgeCaseTrigger',
    enabled: false,
    name: 'feature.edgeCaseTrigger.name',
    description: 'feature.edgeCaseTrigger.description',
  },
  {
    id: 'library',
    enabled: false,
    name: 'feature.library.name',
    description: 'feature.library.description',
  },
  {
    id: 'playlists',
    enabled: false,
    name: 'feature.playlists.name',
    description: 'feature.playlists.description',
  },
  {
    id: 'libraryOverview',
    enabled: false,
    name: 'feature.libraryOverview.name',
    description: 'feature.libraryOverview.description',
  },
  {
    id: 'scanHistory',
    enabled: false,
    name: 'feature.scanHistory.name',
    description: 'feature.scanHistory.description',
  },
  {
    id: 'librarySettings',
    enabled: false,
    name: 'feature.librarySettings.name',
    description: 'feature.librarySettings.description',
  },
  {
    id: 'watchlist',
    enabled: false,
    name: 'feature.watchlist.name',
    description: 'feature.watchlist.description',
  },
]
