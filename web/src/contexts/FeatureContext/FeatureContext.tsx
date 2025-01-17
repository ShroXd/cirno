import {
  ReactNode,
  createContext,
  useCallback,
  useEffect,
  useState,
} from 'react'

import { defaultFeatures } from '~/config/feature'

export interface Feature {
  id: string
  enabled: boolean
  name: string
  description: string
}

export interface FeatureContextType {
  isFeatureEnabled: (featureId: string) => boolean
  toggleFeature: (featureId: string) => void
  resetFeature: () => void
  getAllFeatures: () => readonly Feature[]
}

export const FeatureContext = createContext<FeatureContextType>({
  isFeatureEnabled: () => false,
  toggleFeature: () => {},
  resetFeature: () => {},
  getAllFeatures: () => [],
})

export const FeatureProvider = ({ children }: { children: ReactNode }) => {
  const [features, setFeatures] = useState<Feature[]>(() => {
    const savedFeatures = localStorage.getItem('features')
    return savedFeatures ? JSON.parse(savedFeatures) : defaultFeatures
  })

  useEffect(() => {
    localStorage.setItem('features', JSON.stringify(features))
  }, [features])

  const isFeatureEnabled = useCallback(
    (featureId: string) =>
      features.find(feature => feature.id === featureId)?.enabled ?? false,
    [features]
  )

  const toggleFeature = useCallback(
    (featureId: string) =>
      setFeatures(prevFeatures =>
        prevFeatures.map(feature =>
          feature.id === featureId
            ? { ...feature, enabled: !feature.enabled }
            : feature
        )
      ),
    []
  )

  const resetFeature = useCallback(() => setFeatures(defaultFeatures), [])
  const getAllFeatures = useCallback(() => Object.freeze(features), [features])

  return (
    <FeatureContext.Provider
      value={{
        isFeatureEnabled,
        toggleFeature,
        resetFeature,
        getAllFeatures,
      }}
    >
      {children}
    </FeatureContext.Provider>
  )
}
