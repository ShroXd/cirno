import { ReactNode, createContext, useEffect, useState } from 'react'

import { defaultFeatures } from '~/config/feature'

export interface Feature {
  id: string
  enabled: boolean
  name: string
  description: string
}

export interface FeatureContextType {
  features: Feature[]
  isFeatureEnabled: (featureId: string) => boolean
  toggleFeature: (featureId: string) => void
  updateFeature: (features: Feature[]) => void
}

export const FeatureContext = createContext<FeatureContextType>({
  features: [],
  isFeatureEnabled: () => false,
  toggleFeature: () => {},
  updateFeature: () => {},
})

export const FeatureProvider = ({ children }: { children: ReactNode }) => {
  const [features, setFeatures] = useState<Feature[]>(() => {
    const savedFeatures = localStorage.getItem('features')
    return savedFeatures ? JSON.parse(savedFeatures) : defaultFeatures
  })

  useEffect(() => {
    localStorage.setItem('features', JSON.stringify(features))
  }, [features])

  const isFeatureEnabled = (featureId: string) =>
    features.find(feature => feature.id === featureId)?.enabled ?? false

  const toggleFeature = (featureId: string) => {
    setFeatures(prevFeatures =>
      prevFeatures.map(feature =>
        feature.id === featureId
          ? { ...feature, enabled: !feature.enabled }
          : feature
      )
    )
  }

  const updateFeature = (features: Feature[]) => {
    setFeatures(features)
  }

  return (
    <FeatureContext.Provider
      value={{ features, isFeatureEnabled, toggleFeature, updateFeature }}
    >
      {children}
    </FeatureContext.Provider>
  )
}
