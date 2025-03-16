import { isValidElement } from 'react'

import { Skeleton } from '../ui/skeleton'

export const SkeletonSwitcher = ({
  isLoading,
  children,
  className = '',
}: {
  children: React.ReactNode
  isLoading: boolean
  className?: string
}) => {
  const childClassName = isValidElement(children)
    ? children.props.className || ''
    : ''

  const combinedClassName = `${childClassName} ${className}`.trim()

  return (
    <>{isLoading ? <Skeleton className={combinedClassName} /> : children}</>
  )
}

export const HideOnLoading = ({
  isLoading,
  children,
}: {
  isLoading: boolean
  children: React.ReactNode
}) => {
  return <>{isLoading ? null : children}</>
}
