import { motion } from 'motion/react'

import { cn } from '~/lib/utils'

interface FadeTransitionContainerProps {
  className?: string
  children: React.ReactNode
}

export const FadeTransitionContainer = ({
  children,
  className,
}: FadeTransitionContainerProps) => (
  <motion.div
    initial={{ opacity: 0 }}
    animate={{ opacity: 1 }}
    exit={{ opacity: 0 }}
    transition={{ duration: 0.7, ease: 'easeInOut' }}
    className={cn(
      'relative mb-8 aspect-[21/9] w-full overflow-hidden rounded-t-xl',
      className
    )}
  >
    {children}
  </motion.div>
)
