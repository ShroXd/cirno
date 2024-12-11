import { useLocation } from 'react-router-dom'
import { usePost } from '../../hooks/usePost'
import VideoPlayer from '../../components/VideoPlayer/VideoPlayer'
import { Button } from '@material-tailwind/react'
import { useState } from 'react'

export const Video = () => {
  const location = useLocation()
  const post = usePost()
  const [isMediaReady, setIsMediaReady] = useState(false)

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

  const handlePlay = () => {
    post('/video-player/play', {
      path: location.state?.episode.video_file_path,
    })

    // TODO: This is a temporary hack to wait for the media to be ready
    // After integrating with the notification system, we can properly wait for media ready events
    setTimeout(() => {
      setIsMediaReady(true)
    }, 4000)
  }

  return (
    <div className='w-full h-full'>
      {isMediaReady && <VideoPlayer options={videoJsOptions} />}
      <Button onClick={handlePlay}>Play</Button>
    </div>
  )
}
