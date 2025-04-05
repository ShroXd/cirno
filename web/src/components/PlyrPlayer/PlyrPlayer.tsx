import { useEffect, useRef } from 'react'

import Plyr, { APITypes, PlyrProps } from 'plyr-react'
import 'plyr-react/plyr.css'
import videojs from 'video.js'
import 'video.js/dist/video-js.css'

import './style.css'

const PlyrPlayer = () => {
  const ref = useRef<APITypes | null>(null)
  const playerRef = useRef<any>(null)
  const videoRef = useRef<HTMLVideoElement | null>(null)

  useEffect(() => {
    return () => {
      const player = playerRef.current
      if (player && !player.isDisposed()) {
        player.dispose()
        playerRef.current = null
      }
    }
  }, [])

  useEffect(() => {
    const loadVideo = () => {
      const video = document.getElementById('plyr') as HTMLVideoElement
      videoRef.current = video

      // Initialize video.js
      const player = videojs(video, {
        autoplay: true,
        controls: false, // Let Plyr handle the controls
        sources: [{ src: '/hls/playlist.m3u8', type: 'application/x-mpegURL' }],
        html5: {
          hls: {
            overrideNative: true,
            maxBufferLength: 30,
            maxMaxBufferLength: 60,
            enableLowInitialPlaylist: true,
            smoothQualityChange: true,
            handleManifestRedirects: true,
          },
        },
      })

      playerRef.current = player

      // Connect video.js to Plyr
      // @ts-ignore
      ref.current!.plyr.media = video

      player.ready(() => {
        player.play()
      })

      player.on('error', () => {
        // Handle errors
        const error = player.error()
        console.error('Video.js error:', error)

        // Simple retry logic
        if (error && error.code === 4) {
          setTimeout(() => {
            player.src({
              src: '/hls/playlist.m3u8',
              type: 'application/x-mpegURL',
            })
            player.load()
            player.play()
          }, 1000)
        }
      })
    }

    loadVideo()
  }, [])

  return (
    <Plyr
      id='plyr'
      options={{ volume: 0.1 }}
      source={{} as PlyrProps['source']}
      ref={ref}
    />
  )
}

export default PlyrPlayer
