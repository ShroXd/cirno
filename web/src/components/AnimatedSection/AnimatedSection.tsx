import { useEffect, useRef } from 'react'

import { motion, useAnimation, useInView } from 'motion/react'

interface AnimatedSectionProps {
  children: React.ReactNode
  delay?: number
  className?: string
}

export const AnimatedSection = ({
  children,
  delay = 0,
  className = '',
}: AnimatedSectionProps) => {
  const controls = useAnimation()
  const ref = useRef(null)
  const inView = useInView(ref, { once: true, amount: 0.2 })

  useEffect(() => {
    if (inView) {
      controls.start('visible')
    }
  }, [controls, inView])

  return (
    <motion.section
      ref={ref}
      initial='hidden'
      animate={controls}
      variants={{
        hidden: { opacity: 0, y: 30 },
        visible: {
          opacity: 1,
          y: 0,
          transition: {
            duration: 0.6,
            ease: [0.22, 1, 0.36, 1],
            delay,
          },
        },
      }}
      className={className}
    >
      {children}
    </motion.section>
  )
}
