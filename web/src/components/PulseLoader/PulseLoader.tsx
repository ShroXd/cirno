import { motion } from 'motion/react'

export const PulseLoader = () => (
  <motion.div
    className='bg-zinc-950 absolute inset-0 flex h-full w-full flex-col items-center justify-center rounded-lg'
    initial={{ opacity: 0 }}
    animate={{ opacity: 1 }}
    exit={{ opacity: 0 }}
    transition={{ duration: 0.7, ease: 'easeInOut' }}
  >
    <motion.div className='relative'>
      {/* Glow effect */}
      <motion.div
        className='bg-violet-500/30 absolute inset-0 rounded-full blur-2xl'
        animate={{
          scale: [1, 1.2, 1],
          opacity: [0.3, 0.7, 0.3],
        }}
        transition={{
          duration: 2,
          repeat: Infinity,
          ease: 'easeInOut',
        }}
      />

      {/* Animated bars */}
      <motion.div className='relative flex space-x-4'>
        {[...Array(3)].map((_, i) => (
          <motion.div
            key={i}
            className='from-violet-500 to-violet-300 h-12 w-3 rounded-full bg-gradient-to-t'
            initial={{ scaleY: 0.5, opacity: 0.5 }}
            animate={{
              scaleY: [0.5, 1, 0.5],
              opacity: [0.5, 1, 0.5],
            }}
            transition={{
              duration: 1.5,
              repeat: Infinity,
              delay: i * 0.15,
              ease: 'easeInOut',
            }}
          />
        ))}
      </motion.div>
    </motion.div>

    <motion.p
      className='text-zinc-400 mt-6 text-sm font-light tracking-wide'
      initial={{ opacity: 0 }}
      animate={{
        opacity: [0, 1, 0],
      }}
      transition={{
        duration: 2,
        repeat: Infinity,
        ease: 'easeInOut',
      }}
    >
      Preparing your experience
    </motion.p>
  </motion.div>
)
