import { Feature } from '~/contexts/FeatureContext/FeatureContext'

export const defaultFeatures: Feature[] = [
  {
    id: 'error-alert',
    enabled: false,
    name: 'Error Alert',
    description:
      'Show error alert triggered by global error boundary component',
  },
]
