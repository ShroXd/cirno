import { useCallback, useRef, useState } from 'react'
import { Link, useParams } from 'react-router-dom'

import {
  ArrowLeft,
  Calendar,
  Clock,
  Play,
  Plus,
  Share,
  Star,
  ThumbsUp,
} from 'lucide-react'
import { AnimatePresence, cubicBezier, motion } from 'motion/react'

import { EpisodeDto } from '~/bindings/EpisodeDto'
import { MediaItemDto } from '~/bindings/MediaItemDto'
import { AnimatedSection } from '~/components/AnimatedSection/AnimatedSection'
import { PulseLoader } from '~/components/PulseLoader/PulseLoader'
import {
  HideOnLoading,
  SkeletonSwitcher,
} from '~/components/TransitionSwitcher/TransitionSwitcher'
import VideoPlayer from '~/components/VideoPlayer/VideoPlayer'
import { Badge } from '~/components/ui/badge'
import { Button } from '~/components/ui/button'
import { Card } from '~/components/ui/card'
import { SidebarTrigger } from '~/components/ui/sidebar'
import { useEventBus } from '~/hooks/useEventBus'
import { useFetch } from '~/hooks/useFetch'
import { usePost } from '~/hooks/usePost'

enum VideoPlayerState {
  Idle = 'idle',
  Loading = 'loading',
  Playing = 'playing',
  Paused = 'paused',
  Ended = 'ended',
}

const renderWithTransition = (
  delay: number,
  className: string,
  children: React.ReactNode
) => {
  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.5 }}
      className={className}
    >
      {children}
    </motion.div>
  )
}

export default function ContentDetailPage() {
  const [isWaitingForHlsStream, setIsWaitingForHlsStream] = useState(false)
  const [videoPlaying, setVideoPlaying] = useState(false)

  const [videoPlayerState, setVideoPlayerState] = useState<VideoPlayerState>(
    VideoPlayerState.Idle
  )

  const playerResetRef = useRef<(() => void) | null>(null)
  const handlePlayerReset = useCallback((resetFn: () => void) => {
    playerResetRef.current = resetFn
  }, [])

  const { id } = useParams<{ id: string }>()
  const post = usePost()
  const { onEvent } = useEventBus()

  const {
    data: media,
    error: mediaError,
    isLoading: isMediaLoading,
  } = useFetch<MediaItemDto>(`/media/${id}`)

  const {
    data: episodes,
    error: episodesError,
    isLoading: isEpisodesLoading,
  } = useFetch<EpisodeDto[]>(`/media/${id}/episodes`)

  const videoJsOptions = {
    controls: true,
    autoplay: true,
    responsive: true,
    preload: 'none',
    sources: [
      {
        src: '/hls/playlist.m3u8',
        type: 'application/x-mpegURL',
      },
    ],
  }

  const onBack = () => {
    post('/video-player/stop')
  }

  const handlePlay = useCallback(
    async (episode?: EpisodeDto) => {
      if (!episode) return

      if (playerResetRef.current && videoPlaying) {
        playerResetRef.current()
        await post('/video-player/stop')
      }

      setIsWaitingForHlsStream(true)
      post('/video-player/play', {
        path: episode.video_file_path,
      })

      onEvent('HlsStreamInitialized', () => {
        console.log('HlsStreamInitialized')
        setVideoPlaying(true)
      })
    },
    [post, onEvent, playerResetRef, videoPlaying]
  )

  if (mediaError || episodesError) {
    return (
      <div className='container mx-auto px-4 py-12 text-center'>
        <h1 className='mb-4 text-3xl font-bold'>Content Not Found</h1>
        <p>The content you're looking for doesn't exist or has been removed.</p>
      </div>
    )
  }

  const renderVideoPlayer = () => {
    switch (videoPlayerState) {
      case VideoPlayerState.Loading:
        return (
          <AnimatedSection delay={0.5} className='absolute inset-0'>
            <PulseLoader />
          </AnimatedSection>
        )
      case VideoPlayerState.Playing:
        return (
          <AnimatedSection
            delay={0.5}
            className='absolute inset-0'
            key='video-player'
          >
            <VideoPlayer options={videoJsOptions} onReset={handlePlayerReset} />
          </AnimatedSection>
        )
      default:
        return renderWithTransition(
          0,
          'relative mb-8 aspect-[21/9] w-full overflow-hidden rounded-t-xl',
          <>
            <SkeletonSwitcher
              isLoading={isMediaLoading}
              className='absolute inset-0 h-full w-full rounded-t-xl'
            >
              <img
                src={media?.fanart_path || '/placeholder.svg'}
                alt={media?.title}
                className='absolute inset-0 h-full w-full rounded-t-xl object-cover'
              />
            </SkeletonSwitcher>
            <HideOnLoading isLoading={isMediaLoading}>
              <div className='absolute inset-0 bg-gradient-to-t from-background via-background/30 to-transparent' />
            </HideOnLoading>
            <HideOnLoading isLoading={isMediaLoading}>
              <div className='absolute inset-0 flex items-center justify-center opacity-70'>
                <Button
                  size='lg'
                  className='h-16 w-16 rounded-full p-0'
                  onClick={() => handlePlay(episodes?.[0])}
                >
                  <Play className='h-12 w-12' />
                </Button>
              </div>
            </HideOnLoading>
          </>
        )
    }
  }

  return (
    <div className='flex h-full flex-col bg-background md:px-6'>
      <header className='sticky top-0 z-10 border-b border-border/40 bg-background/80 backdrop-blur-md'>
        <div className='container flex h-16 items-center px-4'>
          <SidebarTrigger className='mr-4 md:hidden' />
          <Button variant='ghost' size='icon' onClick={onBack} asChild>
            <Link to='/'>
              <ArrowLeft className='h-5 w-5' />
              <span className='sr-only'>Back</span>
            </Link>
          </Button>
          <h1 className='ml-4 text-lg font-medium'>Details</h1>
        </div>
      </header>

      <main className='container mx-auto flex-1 overflow-y-auto px-4 py-6'>
        {renderVideoPlayer()}
        {/* {isWaitingForHlsStream ? (
          <div className='relative mb-8 aspect-[21/9] w-full overflow-hidden rounded-t-xl'>
            <AnimatePresence mode='wait'>
              {videoPlaying ? (
                <motion.div
                  key='video-player'
                  className='absolute inset-0'
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  exit={{ opacity: 0 }}
                  transition={{ duration: 0.7, ease: 'easeInOut' }}
                >
                  <VideoPlayer
                    options={videoJsOptions}
                    onReset={handlePlayerReset}
                  />
                </motion.div>
              ) : (
                <motion.div
                  key='pulse-loader'
                  className='absolute inset-0'
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  exit={{ opacity: 0 }}
                  transition={{ duration: 0.7, ease: 'easeInOut' }}
                >
                  <PulseLoader />
                </motion.div>
              )}
            </AnimatePresence>
          </div>
        ) : (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5 }}
            className='relative mb-8 aspect-[21/9] w-full overflow-hidden rounded-t-xl'
          >
            <SkeletonSwitcher
              isLoading={isMediaLoading}
              className='absolute inset-0 h-full w-full rounded-t-xl'
            >
              <img
                src={media?.fanart_path || '/placeholder.svg'}
                alt={media?.title}
                className='absolute inset-0 h-full w-full rounded-t-xl object-cover'
              />
            </SkeletonSwitcher>
            <HideOnLoading isLoading={isMediaLoading}>
              <div className='absolute inset-0 bg-gradient-to-t from-background via-background/30 to-transparent' />
            </HideOnLoading>
            <HideOnLoading isLoading={isMediaLoading}>
              <div className='absolute inset-0 flex items-center justify-center opacity-70'>
                <Button
                  size='lg'
                  className='h-16 w-16 rounded-full p-0'
                  onClick={() => handlePlay(episodes?.[0])}
                >
                  <Play className='h-12 w-12' />
                </Button>
              </div>
            </HideOnLoading>
          </motion.div>
        )} */}

        <motion.div
          variants={{
            hidden: { opacity: 0 },
            visible: {
              opacity: 1,
              transition: {
                staggerChildren: 0.1,
              },
            },
          }}
          initial='hidden'
          animate={isMediaLoading ? 'hidden' : 'visible'}
          className='mb-12 grid grid-cols-1 gap-8 lg:grid-cols-3'
        >
          <div className='lg:col-span-2'>
            <motion.div
              variants={{
                hidden: { y: 20, opacity: 0 },
                visible: {
                  y: 0,
                  opacity: 1,
                  transition: {
                    type: 'spring',
                    stiffness: 100,
                    damping: 15,
                  },
                },
              }}
              className='overflow-y-auto'
            >
              <SkeletonSwitcher
                isLoading={isMediaLoading}
                className='mb-2 h-[2.5rem] w-full'
              >
                <h1 className='mb-2 text-3xl font-bold md:text-4xl'>
                  {media?.title}
                </h1>
              </SkeletonSwitcher>
              <SkeletonSwitcher
                isLoading={isMediaLoading}
                className='h-6 w-[300px]'
              >
                <div className='mb-4 flex flex-wrap items-center gap-3 text-sm text-muted-foreground'>
                  <span className='flex items-center gap-1'>
                    <Calendar className='h-4 w-4' /> {media?.year}
                  </span>
                  <span className='flex items-center gap-1'>
                    <Clock className='h-4 w-4' /> {media?.runtime?.toString()}
                  </span>
                  <span className='flex items-center gap-1'>
                    <Star className='h-4 w-4 text-yellow-500' />{' '}
                    {media?.rating?.toFixed(1)}
                  </span>
                  <Badge variant='outline'>Movie</Badge>
                </div>
              </SkeletonSwitcher>
              <SkeletonSwitcher
                isLoading={isMediaLoading}
                className='h-6 w-[200px]'
              >
                <div className='mb-4 flex flex-wrap gap-2'>
                  {media?.genres?.map(genre => (
                    <Badge key={genre} variant='secondary'>
                      {genre}
                    </Badge>
                  ))}
                </div>
              </SkeletonSwitcher>

              <SkeletonSwitcher
                isLoading={isMediaLoading}
                className='h-[100px] w-full'
              >
                <p className='mb-6 text-muted-foreground'>{media?.plot}</p>
              </SkeletonSwitcher>
              <HideOnLoading isLoading={isMediaLoading}>
                <div className='flex flex-wrap gap-3'>
                  <Button
                    onClick={() => handlePlay(episodes?.[0])}
                    size='lg'
                    className='gap-2'
                  >
                    <Play className='h-4 w-4' /> Watch Now
                  </Button>
                  <Button variant='outline' size='lg' className='gap-2'>
                    <Plus className='h-4 w-4' /> Add to Watchlist
                  </Button>
                  <Button variant='ghost' size='icon' className='ml-auto'>
                    <Share className='h-5 w-5' />
                    <span className='sr-only'>Share</span>
                  </Button>
                  <Button variant='ghost' size='icon'>
                    <ThumbsUp className='h-5 w-5' />
                    <span className='sr-only'>Like</span>
                  </Button>
                </div>
              </HideOnLoading>
            </motion.div>
          </div>

          <div>
            <motion.div
              variants={{
                hidden: { y: 20, opacity: 0 },
                visible: {
                  y: 0,
                  opacity: 1,
                  transition: {
                    type: 'spring',
                    stiffness: 100,
                    damping: 15,
                  },
                },
              }}
              className='space-y-4'
            >
              <div>
                <h3 className='mb-1 text-lg font-semibold'>Original Title</h3>
                <SkeletonSwitcher
                  isLoading={isMediaLoading}
                  className='h-6 w-full'
                >
                  <p className='text-muted-foreground'>
                    {media?.original_title}
                  </p>
                </SkeletonSwitcher>
              </div>
              <div>
                <h3 className='mb-1 text-lg font-semibold'>Country</h3>
                <SkeletonSwitcher
                  isLoading={isMediaLoading}
                  className='h-6 w-full'
                >
                  <p className='text-muted-foreground'>{media?.country}</p>
                </SkeletonSwitcher>
              </div>
              <div>
                <h3 className='mb-1 text-lg font-semibold'>Studio</h3>
                <SkeletonSwitcher
                  isLoading={isMediaLoading}
                  className='h-6 w-full'
                >
                  <div className='text-muted-foreground'>
                    {media?.studios.map(studio => (
                      <Badge
                        className='mb-1 mr-1'
                        key={studio}
                        variant='outline'
                      >
                        {studio}
                      </Badge>
                    ))}
                  </div>
                </SkeletonSwitcher>
              </div>
              <div>
                <h3 className='mb-1 text-lg font-semibold'>Actors</h3>
                <SkeletonSwitcher
                  isLoading={isMediaLoading}
                  className='h-6 w-full'
                >
                  <div className='grid grid-cols-2 gap-2'>
                    {media?.actors.map(actor => (
                      <div key={actor.name} className='flex items-center gap-2'>
                        <div className='relative h-8 w-8 overflow-hidden rounded-full'>
                          <img
                            src={actor.thumb || '/placeholder.svg'}
                            alt={actor.name || ''}
                            className='h-full w-full object-cover'
                          />
                        </div>
                        <div className='overflow-hidden'>
                          <p className='truncate text-sm font-medium'>
                            {actor.name}
                          </p>
                          <p className='truncate text-xs text-muted-foreground'>
                            {actor.role}
                          </p>
                        </div>
                      </div>
                    ))}
                  </div>
                </SkeletonSwitcher>
              </div>
            </motion.div>
          </div>
        </motion.div>

        <motion.div
          variants={{
            hidden: { opacity: 0 },
            visible: {
              opacity: 1,
              transition: {
                staggerChildren: 0.1,
              },
            },
          }}
          initial='hidden'
          animate={isEpisodesLoading ? 'hidden' : 'visible'}
        >
          <div className='space-y-4'>
            <SkeletonSwitcher
              isLoading={isEpisodesLoading}
              className='h-32 w-full'
            >
              {episodes?.map(episode => (
                <motion.div
                  key={episode.title}
                  variants={{
                    hidden: { y: 20, opacity: 0 },
                    visible: {
                      y: 0,
                      opacity: 1,
                      transition: {
                        type: 'spring',
                        stiffness: 100,
                        damping: 15,
                      },
                    },
                  }}
                >
                  <Card className='overflow-hidden'>
                    <div className='flex flex-col sm:flex-row'>
                      <div className='relative aspect-video w-full sm:aspect-[16/9] sm:w-48'>
                        <img
                          src={episode.thumb_image || '/placeholder.svg'}
                          alt={episode.title || 'Episode'}
                          className='absolute inset-0 h-full w-full rounded-l-xl object-cover'
                        />
                        <div className='absolute inset-0 flex items-center justify-center bg-black/50 opacity-0 transition-opacity hover:opacity-100'>
                          <Button
                            size='sm'
                            variant='secondary'
                            onClick={() => handlePlay(episode)}
                          >
                            <Play className='mr-1 h-4 w-4' /> Play
                          </Button>
                        </div>
                      </div>
                      <div className='flex-1 p-4'>
                        <div className='mb-2 flex items-center justify-between'>
                          <div>
                            <h3 className='font-medium'>
                              {episode.episode_number !== null
                                ? episode.episode_number.toString() +
                                  '. ' +
                                  episode.title
                                : episode.title}
                            </h3>
                            <p className='text-sm text-muted-foreground'>
                              20 min
                            </p>
                          </div>
                          <Button
                            variant='ghost'
                            size='sm'
                            className='sm:hidden'
                          >
                            <Play className='h-4 w-4' />
                          </Button>
                        </div>
                        <p className='text-sm text-muted-foreground'>
                          {episode.plot}
                        </p>
                      </div>
                    </div>
                  </Card>
                </motion.div>
              ))}
            </SkeletonSwitcher>
          </div>
        </motion.div>
      </main>
    </div>
  )
}
