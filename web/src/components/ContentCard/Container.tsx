import { ReactNode } from 'react'

import { Card } from '@material-tailwind/react'

interface ContentCardContainerProps {
  children: ReactNode
  className?: string
}

export const ContentCardContainer = ({
  children,
  className,
}: ContentCardContainerProps) => (
  <Card
    className={`flex select-none flex-col overflow-hidden rounded-xl bg-white shadow-lg ${className || ''}`}
  >
    {children}
  </Card>
)
