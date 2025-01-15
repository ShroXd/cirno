import { ReactNode } from 'react'

import { isEmpty as defaultIsEmpty } from 'lodash'

import { DefaultEmpty } from './DefaultEdgeComponents/DefaultEmpty'
import { DefaultError } from './DefaultEdgeComponents/DefaultError'
import { DefaultLoading } from './DefaultEdgeComponents/DefaultLoading'

interface AsyncSwitcherProps<T> {
  children: ReactNode
  loading?: boolean
  error?: Error | null
  data?: T | null
  loadingComponent?: ReactNode
  errorComponent?: ReactNode
  emptyComponent?: ReactNode
  isEmpty?: (data: T | undefined | null) => boolean
}

export const AsyncSwitcher = <T extends object>({
  children,
  loading,
  error,
  data,
  loadingComponent,
  errorComponent,
  emptyComponent,
  isEmpty = defaultIsEmpty,
}: AsyncSwitcherProps<T>) => {
  if (loading) return loadingComponent || <DefaultLoading />
  if (error) return errorComponent || <DefaultError error={error} />
  if (isEmpty && isEmpty(data)) return emptyComponent || <DefaultEmpty />

  return children
}
