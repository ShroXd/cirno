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
    className={cn(className)}
  >
    {children}
  </motion.div>
)

interface FadeInUpTransitionContainerProps {
  className?: string
  children: React.ReactNode
  delay?: number
  duration?: number
}

export const FadeInUpTransitionContainer = ({
  children,
  className,
  delay = 0,
  duration = 0.5,
}: FadeInUpTransitionContainerProps) => (
  <motion.div
    initial={{ opacity: 0, y: 20 }}
    animate={{ opacity: 1, y: 0 }}
    transition={{
      duration,
      delay,
      ease: 'easeOut',
    }}
    className={cn(className)}
  >
    {children}
  </motion.div>
)
