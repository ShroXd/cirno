import { motion } from 'motion/react'

import { cn } from '~/lib/utils'

interface FadeInProps {
  className?: string
  children: React.ReactNode
}

export const FadeIn = ({ children, className }: FadeInProps) => (
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
