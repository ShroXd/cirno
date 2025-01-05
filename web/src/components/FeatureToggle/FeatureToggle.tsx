import { ReactNode } from 'react'

import { useFeatureFlag } from '~/hooks/feature/useFeatureFlag'

interface FeatureToggleProps {
  featureId: string
  children?: ReactNode
  fallback?: ReactNode
}

export const FeatureToggle = ({
  featureId,
  children,
  fallback,
}: FeatureToggleProps) => {
  const isEnabled = useFeatureFlag(featureId)

  return isEnabled ? <>{children}</> : <>{fallback}</>
}
