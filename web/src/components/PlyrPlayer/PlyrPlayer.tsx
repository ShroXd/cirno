import { useEffect, useRef } from 'react'

import Hls from 'hls.js'
import Plyr, { APITypes, PlyrInstance, PlyrProps } from 'plyr-react'
import 'plyr-react/plyr.css'

import './style.css'

// Key - segment
// Value - timer and retry count
type RetryTimers = Record<string, { timer: NodeJS.Timeout; count: number }>

const PlyrPlayer = () => {
  const ref = useRef<APITypes | null>(null)
  const hlsRef = useRef<Hls | null>(null)
  const retryTimersRef = useRef<RetryTimers>({})

  useEffect(
    () => () => {
      Object.values(retryTimersRef.current).forEach(({ timer }) =>
        clearTimeout(timer)
      )
      if (hlsRef.current) {
        hlsRef.current.destroy()
      }
    },
    []
  )

  useEffect(() => {
    const retrySegmentLoad = (segmentUrl: string, onSuccess: () => void) => {
      const key = segmentUrl.split('/').pop() || segmentUrl

      if (retryTimersRef.current[key]) {
        clearTimeout(retryTimersRef.current[key].timer)
      }

      const retryCount = (retryTimersRef.current[key]?.count || 0) + 1

      // delay = 1s * 1.3^retryCount
      const delay = Math.min(
        1000 * Math.pow(1.3, Math.min(retryCount - 1, 10)),
        5000
      )

      const timer = setTimeout(() => {
        fetch(segmentUrl, { method: 'HEAD' })
          .then(response => {
            if (response.ok) {
              delete retryTimersRef.current[key]
              onSuccess()
            } else {
              retrySegmentLoad(segmentUrl, onSuccess)
            }
          })
          .catch(() => {
            retrySegmentLoad(segmentUrl, onSuccess)
          })
      }, delay)

      retryTimersRef.current[key] = { timer, count: retryCount }
    }

    const loadVideo = async () => {
      const video = document.getElementById('plyr') as HTMLVideoElement
      const hls = new Hls({
        // Configure HLS.js to handle dynamically generated segments
        maxBufferLength: 30,
        maxMaxBufferLength: 60,
        enableWorker: true,
        lowLatencyMode: true,
        // Set minimal error recovery as we'll handle it ourselves
        maxLoadingDelay: 1,
        fragLoadingMaxRetry: 2,
        fragLoadingRetryDelay: 500,
        manifestLoadingMaxRetry: 1,
        levelLoadingMaxRetry: 1,
      })

      hlsRef.current = hls

      hls.loadSource('/hls/playlist.m3u8')
      hls.attachMedia(video)

      // @ts-ignore
      ref.current!.plyr.media = video

      hls.on(Hls.Events.MANIFEST_PARSED, () => {
        ;(ref.current!.plyr as PlyrInstance).play()
      })

      hls.on(Hls.Events.ERROR, (_event, data) => {
        // Hls segments are generated dynamically, use custom retry logic
        if (data.response && data.response.code === 404) {
          data.fatal = false

          if (
            data.type === Hls.ErrorTypes.NETWORK_ERROR &&
            data.details &&
            (data.details === Hls.ErrorDetails.FRAG_LOAD_ERROR ||
              data.details === Hls.ErrorDetails.FRAG_LOAD_TIMEOUT)
          ) {
            const segmentUrl = data.frag?.url
            if (segmentUrl) {
              retrySegmentLoad(segmentUrl, () => {
                if (hlsRef.current) {
                  hlsRef.current.startLoad()
                }
              })
            }
          }
          return
        }

        if (data.fatal) {
          switch (data.type) {
            case Hls.ErrorTypes.NETWORK_ERROR:
              hls.startLoad()
              break
            case Hls.ErrorTypes.MEDIA_ERROR:
              hls.recoverMediaError()
              break
            default:
              hls.destroy()
              loadVideo()
              break
          }
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
