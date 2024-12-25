/* eslint-disable @typescript-eslint/no-explicit-any */
import { useEffect, useRef } from 'react'
import videojs from 'video.js'
import 'video.js/dist/video-js.css'
import './style.css'

import { useWebSocket } from '@/hooks/useWebSocket'

export const VideoPlayer = (props: any) => {
  const videoRef = useRef<any>(null)
  const playerRef = useRef<any>(null)
  const { options, onReady } = props
  const { sendMessage } = useWebSocket()

  useEffect(() => {
    if (!playerRef.current) {
      const videoElement = document.createElement('video-js')

      videoElement.classList.add('vjs-big-play-centered')
      videoRef.current.appendChild(videoElement)

      const player = (playerRef.current = videojs(videoElement, options, () => {
        videojs.log('player is ready')

        player.play()

        player.on('error', (e: any) => {
          console.log('error', e)
        })

        // eslint-disable-next-line @typescript-eslint/no-unused-expressions
        onReady && onReady(player)
      }))

      player.on('seeking', () => {
        const currentTime = Math.floor(player.currentTime() ?? 0)
        console.log('seeking: ', currentTime)
        // sendMessage({
        //   PipelineAction: {
        //     Seek: currentTime,
        //   },
        // })
      })
    } else {
      const player = playerRef.current

      player.autoplay(options.autoplay)
      player.src(options.sources)
    }
  }, [options, videoRef, onReady, sendMessage])

  // Dispose the Video.js player when the functional component unmounts
  useEffect(() => {
    const player = playerRef.current

    return () => {
      if (player && !player.isDisposed()) {
        player.dispose()
        playerRef.current = null
      }
    }
  }, [playerRef])

  return (
    <div className='player-container' data-vjs-player>
      <div ref={videoRef} />
    </div>
  )
}

export default VideoPlayer
