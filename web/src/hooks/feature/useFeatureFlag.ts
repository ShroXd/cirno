import { useFeatures } from './useFeatures'

export const useFeatureFlag = (featureId: string) => {
  const { isFeatureEnabled } = useFeatures()
  return isFeatureEnabled(featureId)
}
