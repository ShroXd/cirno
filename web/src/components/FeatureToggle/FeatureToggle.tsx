import { ComponentProps, ReactNode } from 'react'

import { useFeatureFlag } from '~/hooks/feature/useFeatureFlag'

type FeatureToggleProps = ComponentProps<'div'> & {
  featureId: string
  children?: ReactNode
  fallback?: ReactNode
}

export const FeatureToggle = ({
  featureId,
  children,
  fallback,
  ...props
}: FeatureToggleProps) => {
  const isEnabled = useFeatureFlag(featureId)
  return <div {...props}>{isEnabled ? children : fallback}</div>
}
