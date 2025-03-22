import { motion } from 'motion/react'

import { cn } from '~/lib/utils'

interface FadeInUpProps {
  className?: string
  children: React.ReactNode
  delay?: number
  duration?: number
}

export const FadeInUp = ({
  children,
  className,
  delay = 0,
  duration = 0.5,
}: FadeInUpProps) => (
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
