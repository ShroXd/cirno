import { useContext } from 'react'

import { FeatureContext } from '~/contexts/FeatureContext/FeatureContext'

export const useFeatures = () => {
  const context = useContext(FeatureContext)
  if (!context) {
    throw new Error('useFeatures must be used within a FeatureProvider')
  }

  return context
}
