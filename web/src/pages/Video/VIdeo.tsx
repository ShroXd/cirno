import { useLocation } from 'react-router-dom'
import VideoPlayer from '../../components/VideoPlayer/VideoPlayer'
import { useEffect } from 'react'
import { usePost } from '../../hooks/usePost'

export const Video = () => {
  const location = useLocation()
  console.log('location: ', location)

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
    post('/video-player/play', {
      path: location.state?.episode.video_file_path,
    })
  }, [location.state?.episode, post])

  return (
    <div className='aspect-w-16 aspect-h-9'>
      <VideoPlayer options={videoJsOptions} />
    </div>
  )
}
