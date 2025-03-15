import VideoPlayer from '~/components/VideoPlayer/VideoPlayer'
import { usePost } from '~/hooks/usePost'

export const Video = () => {
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

  const onBack = () => {
    post('/video-player/stop')
  }

  return (
    <div className='h-full w-full'>
      <VideoPlayer options={videoJsOptions} />
    </div>
  )
}
