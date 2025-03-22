import { isValidElement } from 'react'

import { Skeleton } from '~/components/ui/skeleton'

interface SkeletonSwitcherProps {
  isLoading: boolean
  children: React.ReactNode
  className?: string
}

export const SkeletonSwitcher = ({
  isLoading,
  children,
  className = '',
}: SkeletonSwitcherProps) => {
  const childClassName = isValidElement(children)
    ? children.props.className || ''
    : ''

  const combinedClassName = `${childClassName} ${className}`.trim()

  return (
    <>{isLoading ? <Skeleton className={combinedClassName} /> : children}</>
  )
}
