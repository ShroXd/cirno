import { ReactNode } from 'react'

export const wrapInGrid = (children: ReactNode) => {
  return (
    // TODO: optimize the grid layout for different screen sizes
    <div className='grid grid-cols-1 gap-4 gap-y-12 sm:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 2xl:grid-cols-7'>
      {children}
    </div>
  )
}
