import { useEffect, useRef, useState } from 'react'
import { Link, useNavigate, useParams } from 'react-router-dom'

import {
  ArrowLeft,
  ChevronLeft,
  ChevronRight,
  Maximize,
  Minimize,
  Pause,
  Play,
  Settings,
  SkipBack,
  SkipForward,
  Volume2,
  VolumeX,
  X,
} from 'lucide-react'

import { Button } from '../components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '../components/ui/dropdown-menu'
import { Slider } from '../components/ui/slider'
import { cn } from '../lib/utils'

// Mock data function - in a real app, this would fetch from an API
const getContentForPlayer = (id: string) => {
  // This is simplified mock data for the player
  const allContent = [
    {
      id: '1',
      title: 'Interstellar',
      type: 'movie',
      videoSrc:
        'https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4', // Sample video
      duration: 8580, // 2h 23m in seconds
      thumbnail: '/placeholder.svg?height=600&width=1000',
      nextUp: null,
    },
    {
      id: '2-1-1', // Show ID - Season - Episode
      title: 'Breaking Bad',
      episodeTitle: 'Pilot',
      seasonNumber: 1,
      episodeNumber: 1,
      type: 'tv',
      videoSrc:
        'https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ElephantsDream.mp4', // Sample video
      duration: 3480, // 58m in seconds
      thumbnail: '/placeholder.svg?height=600&width=1000',
      nextUp: {
        id: '2-1-2',
        title: "Cat's in the Bag...",
        thumbnail: '/placeholder.svg?height=300&width=500',
      },
      previousEpisode: null,
    },
    {
      id: '2-1-2', // Show ID - Season - Episode
      title: 'Breaking Bad',
      episodeTitle: "Cat's in the Bag...",
      seasonNumber: 1,
      episodeNumber: 2,
      type: 'tv',
      videoSrc:
        'https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerBlazes.mp4', // Sample video
      duration: 2880, // 48m in seconds
      thumbnail: '/placeholder.svg?height=600&width=1000',
      nextUp: {
        id: '2-1-3',
        title: "...And the Bag's in the River",
        thumbnail: '/placeholder.svg?height=300&width=500',
      },
      previousEpisode: {
        id: '2-1-1',
        title: 'Pilot',
        thumbnail: '/placeholder.svg?height=300&width=500',
      },
    },
    {
      id: '4-1-1', // Show ID - Season - Episode
      title: 'Game of Thrones',
      episodeTitle: 'Winter Is Coming',
      seasonNumber: 1,
      episodeNumber: 1,
      type: 'tv',
      videoSrc:
        'https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/TearsOfSteel.mp4', // Sample video
      duration: 3720, // 62m in seconds
      thumbnail: '/placeholder.svg?height=600&width=1000',
      nextUp: {
        id: '4-1-2',
        title: 'The Kingsroad',
        thumbnail: '/placeholder.svg?height=300&width=500',
      },
      previousEpisode: null,
    },
  ]

  return allContent.find(item => item.id === id)
}

// Format seconds to HH:MM:SS or MM:SS
const formatTime = (seconds: number) => {
  const hours = Math.floor(seconds / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)
  const secs = Math.floor(seconds % 60)

  if (hours > 0) {
    return `${hours}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`
  }

  return `${minutes}:${secs.toString().padStart(2, '0')}`
}

export default function WatchPage() {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const content = getContentForPlayer(id || '')
  const videoRef = useRef<HTMLVideoElement>(null)
  const playerContainerRef = useRef<HTMLDivElement>(null)

  const [isPlaying, setIsPlaying] = useState(false)
  const [currentTime, setCurrentTime] = useState(0)
  const [duration, setDuration] = useState(0)
  const [volume, setVolume] = useState(1)
  const [isMuted, setIsMuted] = useState(false)
  const [isFullscreen, setIsFullscreen] = useState(false)
  const [showControls, setShowControls] = useState(true)
  const [showInfo, setShowInfo] = useState(true)
  const [showNextUp, setShowNextUp] = useState(false)

  const hasContent = !!content

  // Set up event listeners and initial state
  useEffect(() => {
    const video = videoRef.current
    if (!video || !hasContent) return

    const handleTimeUpdate = () => {
      setCurrentTime(video.currentTime)

      // Show next up card when near the end (last 20 seconds)
      if (content.nextUp && video.duration - video.currentTime <= 20) {
        setShowNextUp(true)
      } else {
        setShowNextUp(false)
      }
    }

    const handleLoadedMetadata = () => {
      setDuration(video.duration)
    }

    const handleEnded = () => {
      setIsPlaying(false)
      if (content.nextUp) {
        // Auto-play next episode after 5 seconds
        setTimeout(() => {
          navigate(`/watch/${content.nextUp.id}`)
        }, 5000)
      }
    }

    video.addEventListener('timeupdate', handleTimeUpdate)
    video.addEventListener('loadedmetadata', handleLoadedMetadata)
    video.addEventListener('ended', handleEnded)

    return () => {
      video.removeEventListener('timeupdate', handleTimeUpdate)
      video.removeEventListener('loadedmetadata', handleLoadedMetadata)
      video.removeEventListener('ended', handleEnded)
    }
  }, [content, navigate, hasContent])

  // Handle auto-hide controls
  useEffect(() => {
    let timeout: NodeJS.Timeout | null = null

    const handleMouseMove = () => {
      setShowControls(true)

      // Clear existing timeout
      if (timeout) {
        clearTimeout(timeout)
      }

      // Set new timeout to hide controls after 3 seconds
      timeout = setTimeout(() => {
        if (isPlaying) {
          setShowControls(false)
        }
      }, 3000)
    }

    const playerContainer = playerContainerRef.current
    if (playerContainer) {
      playerContainer.addEventListener('mousemove', handleMouseMove)
    }

    return () => {
      if (playerContainer) {
        playerContainer.removeEventListener('mousemove', handleMouseMove)
      }

      if (timeout) {
        clearTimeout(timeout)
      }
    }
  }, [isPlaying])

  // Handle fullscreen changes
  useEffect(() => {
    const handleFullscreenChange = () => {
      setIsFullscreen(!!document.fullscreenElement)
    }

    document.addEventListener('fullscreenchange', handleFullscreenChange)

    return () => {
      document.removeEventListener('fullscreenchange', handleFullscreenChange)
    }
  }, [])

  // Handle keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      const video = videoRef.current
      if (!video) return

      switch (e.key) {
        case ' ':
        case 'k':
          togglePlay()
          break
        case 'ArrowRight':
          video.currentTime = Math.min(video.currentTime + 10, video.duration)
          break
        case 'ArrowLeft':
          video.currentTime = Math.max(video.currentTime - 10, 0)
          break
        case 'f':
          toggleFullscreen()
          break
        case 'm':
          toggleMute()
          break
        case 'Escape':
          if (isFullscreen) {
            document.exitFullscreen()
          }
          break
      }
    }

    window.addEventListener('keydown', handleKeyDown)

    return () => {
      window.removeEventListener('keydown', handleKeyDown)
    }
  }, [isFullscreen])

  // Player control functions
  const togglePlay = () => {
    const video = videoRef.current
    if (!video) return

    if (isPlaying) {
      video.pause()
    } else {
      video.play()
    }

    setIsPlaying(!isPlaying)
  }

  const toggleMute = () => {
    const video = videoRef.current
    if (!video) return

    video.muted = !isMuted
    setIsMuted(!isMuted)
  }

  const toggleFullscreen = () => {
    const playerContainer = playerContainerRef.current
    if (!playerContainer) return

    if (!isFullscreen) {
      playerContainer.requestFullscreen()
    } else {
      document.exitFullscreen()
    }
  }

  const handleVolumeChange = (value: number[]) => {
    const video = videoRef.current
    if (!video) return

    const newVolume = value[0]
    video.volume = newVolume
    setVolume(newVolume)

    if (newVolume === 0) {
      setIsMuted(true)
      video.muted = true
    } else if (isMuted) {
      setIsMuted(false)
      video.muted = false
    }
  }

  const handleSeek = (value: number[]) => {
    const video = videoRef.current
    if (!video) return

    video.currentTime = value[0]
    setCurrentTime(value[0])
  }

  const handlePlaybackSpeed = (speed: number) => {
    const video = videoRef.current
    if (!video) return

    video.playbackRate = speed
  }

  // Calculate progress percentage
  const progressPercentage = duration ? (currentTime / duration) * 100 : 0

  // Handle video not found
  if (!hasContent) {
    return (
      <div className='flex h-screen flex-col items-center justify-center bg-black'>
        <h1 className='mb-4 text-3xl font-bold'>Video Not Found</h1>
        <p className='mb-8'>
          The video you're looking for doesn't exist or has been removed.
        </p>
        <Button asChild>
          <Link to='/'>Back to Home</Link>
        </Button>
      </div>
    )
  }

  return (
    <div className='fixed inset-0 flex items-center justify-center bg-black'>
      <div
        ref={playerContainerRef}
        className='relative h-full w-full overflow-hidden'
        onClick={togglePlay}
      >
        {/* Video Element */}
        <video
          ref={videoRef}
          src={content.videoSrc}
          className='h-full w-full object-contain'
          poster={content.thumbnail}
          onPlay={() => setIsPlaying(true)}
          onPause={() => setIsPlaying(false)}
        />

        {/* Top Info Bar */}
        <div
          className={cn(
            'absolute left-0 right-0 top-0 bg-gradient-to-b from-black/80 to-transparent p-4 transition-opacity duration-300',
            showControls || !isPlaying
              ? 'opacity-100'
              : 'pointer-events-none opacity-0'
          )}
        >
          <div className='flex items-center'>
            <Button
              variant='ghost'
              size='icon'
              className='mr-2 text-white hover:bg-black/20'
              onClick={e => {
                e.stopPropagation()
                navigate(-1)
              }}
            >
              <ArrowLeft className='h-6 w-6' />
            </Button>

            <div>
              <h1 className='text-lg font-medium text-white'>
                {content.title}
                {content.type === 'tv' && ` - ${content.episodeTitle}`}
              </h1>
              {content.type === 'tv' && (
                <p className='text-sm text-white/70'>
                  Season {content.seasonNumber}, Episode {content.episodeNumber}
                </p>
              )}
            </div>
          </div>
        </div>

        {/* Next Up Card */}
        {showNextUp && content.nextUp && (
          <div
            className='absolute right-8 top-1/2 w-64 -translate-y-1/2 transform overflow-hidden rounded-lg bg-black/80 shadow-lg'
            onClick={e => e.stopPropagation()}
          >
            <div className='relative h-36 w-full'>
              <img
                src={content.nextUp.thumbnail || '/placeholder.svg'}
                alt={content.nextUp.title}
                className='h-full w-full object-cover'
              />
            </div>
            <div className='p-4'>
              <p className='mb-1 text-sm text-white/70'>Up next</p>
              <h3 className='mb-2 font-medium text-white'>
                {content.nextUp.title}
              </h3>
              <div className='flex gap-2'>
                <Button
                  size='sm'
                  className='flex-1'
                  onClick={() => navigate(`/watch/${content.nextUp!.id}`)}
                >
                  Play Next
                </Button>
                <Button
                  size='sm'
                  variant='outline'
                  className='text-white'
                  onClick={() => setShowNextUp(false)}
                >
                  <X className='h-4 w-4' />
                </Button>
              </div>
            </div>
          </div>
        )}

        {/* Episode Navigation */}
        {content.type === 'tv' && (showControls || !isPlaying) && (
          <div className='pointer-events-none absolute left-0 right-0 top-1/2 flex justify-between px-4'>
            {content.previousEpisode && (
              <Button
                variant='ghost'
                size='icon'
                className='pointer-events-auto h-12 w-12 rounded-full bg-black/30 text-white hover:bg-black/50'
                onClick={e => {
                  e.stopPropagation()
                  navigate(`/watch/${content.previousEpisode!.id}`)
                }}
              >
                <ChevronLeft className='h-8 w-8' />
              </Button>
            )}

            {content.nextUp && (
              <Button
                variant='ghost'
                size='icon'
                className='pointer-events-auto ml-auto h-12 w-12 rounded-full bg-black/30 text-white hover:bg-black/50'
                onClick={e => {
                  e.stopPropagation()
                  navigate(`/watch/${content.nextUp!.id}`)
                }}
              >
                <ChevronRight className='h-8 w-8' />
              </Button>
            )}
          </div>
        )}

        {/* Play/Pause Big Button */}
        {!isPlaying && (
          <div className='absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 transform'>
            <Button
              variant='ghost'
              size='icon'
              className='h-20 w-20 rounded-full bg-black/30 text-white hover:bg-black/50'
              onClick={togglePlay}
            >
              <Play className='ml-1 h-12 w-12' />
            </Button>
          </div>
        )}

        {/* Bottom Controls */}
        <div
          className={cn(
            'absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/80 to-transparent p-4 transition-opacity duration-300',
            showControls || !isPlaying
              ? 'opacity-100'
              : 'pointer-events-none opacity-0'
          )}
          onClick={e => e.stopPropagation()}
        >
          {/* Progress Bar */}
          <div className='mb-2 px-1'>
            <Slider
              value={[currentTime]}
              max={duration}
              step={0.1}
              onValueChange={handleSeek}
              className='cursor-pointer [&>span:first-child]:h-1.5 [&>span:first-child]:bg-white/30 [&>span:first-child_span]:bg-primary [&_[role=slider]]:h-4 [&_[role=slider]]:w-4 [&_[role=slider]]:border-0 [&_[role=slider]]:bg-white [&_[role=slider]]:opacity-0 hover:[&_[role=slider]]:opacity-100'
            />
          </div>

          <div className='flex items-center justify-between'>
            <div className='flex items-center gap-2'>
              {/* Play/Pause Button */}
              <Button
                variant='ghost'
                size='icon'
                className='text-white hover:bg-white/10'
                onClick={togglePlay}
              >
                {isPlaying ? (
                  <Pause className='h-5 w-5' />
                ) : (
                  <Play className='h-5 w-5' />
                )}
              </Button>

              {/* Skip Buttons */}
              <Button
                variant='ghost'
                size='icon'
                className='text-white hover:bg-white/10'
                onClick={e => {
                  e.stopPropagation()
                  if (videoRef.current) {
                    videoRef.current.currentTime = Math.max(
                      videoRef.current.currentTime - 10,
                      0
                    )
                  }
                }}
              >
                <SkipBack className='h-5 w-5' />
              </Button>

              <Button
                variant='ghost'
                size='icon'
                className='text-white hover:bg-white/10'
                onClick={e => {
                  e.stopPropagation()
                  if (videoRef.current) {
                    videoRef.current.currentTime = Math.min(
                      videoRef.current.currentTime + 10,
                      duration
                    )
                  }
                }}
              >
                <SkipForward className='h-5 w-5' />
              </Button>

              {/* Volume Control */}
              <div className='flex items-center gap-1'>
                <Button
                  variant='ghost'
                  size='icon'
                  className='text-white hover:bg-white/10'
                  onClick={toggleMute}
                >
                  {isMuted || volume === 0 ? (
                    <VolumeX className='h-5 w-5' />
                  ) : (
                    <Volume2 className='h-5 w-5' />
                  )}
                </Button>

                <div className='hidden w-24 sm:block'>
                  <Slider
                    value={[isMuted ? 0 : volume]}
                    max={1}
                    step={0.01}
                    onValueChange={handleVolumeChange}
                    className='cursor-pointer [&>span:first-child]:h-1 [&>span:first-child]:bg-white/30 [&>span:first-child_span]:bg-white [&_[role=slider]]:h-3 [&_[role=slider]]:w-3 [&_[role=slider]]:border-0 [&_[role=slider]]:bg-white'
                  />
                </div>
              </div>

              {/* Time Display */}
              <div className='ml-2 text-sm text-white'>
                {formatTime(currentTime)} / {formatTime(duration)}
              </div>
            </div>

            <div className='flex items-center gap-2'>
              {/* Playback Speed */}
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button
                    variant='ghost'
                    size='icon'
                    className='text-white hover:bg-white/10'
                  >
                    <Settings className='h-5 w-5' />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align='end'>
                  <DropdownMenuItem onClick={() => handlePlaybackSpeed(0.5)}>
                    0.5x
                  </DropdownMenuItem>
                  <DropdownMenuItem onClick={() => handlePlaybackSpeed(0.75)}>
                    0.75x
                  </DropdownMenuItem>
                  <DropdownMenuItem onClick={() => handlePlaybackSpeed(1)}>
                    Normal
                  </DropdownMenuItem>
                  <DropdownMenuItem onClick={() => handlePlaybackSpeed(1.25)}>
                    1.25x
                  </DropdownMenuItem>
                  <DropdownMenuItem onClick={() => handlePlaybackSpeed(1.5)}>
                    1.5x
                  </DropdownMenuItem>
                  <DropdownMenuItem onClick={() => handlePlaybackSpeed(2)}>
                    2x
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>

              {/* Fullscreen Toggle */}
              <Button
                variant='ghost'
                size='icon'
                className='text-white hover:bg-white/10'
                onClick={toggleFullscreen}
              >
                {isFullscreen ? (
                  <Minimize className='h-5 w-5' />
                ) : (
                  <Maximize className='h-5 w-5' />
                )}
              </Button>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
