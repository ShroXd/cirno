import { useEffect } from 'react'
import { useLocation } from 'react-router-dom'
import { usePost } from '../../hooks/usePost'
import VideoPlayer from '../../components/VideoPlayer/VideoPlayer'

export const Video = () => {
  const location = useLocation()
  const post = usePost()

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

  useEffect(() => {
    const initializePlayer = async () => {
      await post('/video-player/play', {
        path: location.state?.episode.video_file_path,
      })
    }

    if (location.state?.episode) {
      initializePlayer()
    }
  }, [])

  return <VideoPlayer options={videoJsOptions} />
}
