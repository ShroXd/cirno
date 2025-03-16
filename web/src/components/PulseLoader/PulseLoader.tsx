import { motion } from 'motion/react'

export const PulseLoader = () => (
  <motion.div
    className='absolute inset-0 flex h-full w-full flex-col items-center justify-center rounded-lg bg-zinc-950'
    initial={{ opacity: 0 }}
    animate={{ opacity: 1 }}
    exit={{ opacity: 0 }}
    transition={{ duration: 0.7, ease: 'easeInOut' }}
  >
    <motion.div className='relative'>
      {/* Animated bars */}
      <motion.div className='relative flex space-x-4'>
        {[...Array(3)].map((_, i) => (
          <motion.div
            key={i}
            className='h-12 w-3 rounded-full bg-gradient-to-t from-stone-500 to-slate-200'
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

    {/* Text */}
    <motion.p
      className='mt-6 text-sm font-light tracking-wide text-zinc-400'
      initial={{ opacity: 0 }}
      animate={{
        opacity: [0, 1, 0],
      }}
      transition={{
        duration: 3,
        repeat: Infinity,
        ease: 'easeInOut',
      }}
    >
      Preparing your experience
    </motion.p>
  </motion.div>
)
