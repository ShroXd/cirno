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
]
