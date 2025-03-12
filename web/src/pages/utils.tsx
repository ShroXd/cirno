import { ReactNode } from 'react'

export const wrapInGrid = (children: ReactNode) => (
  <div className='grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6'>
    {children}
  </div>
)
