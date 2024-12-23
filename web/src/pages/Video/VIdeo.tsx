import Breadcrumb from '@/components/Breadcrumb/Breadcrumb'
import { Container } from '@/components/Container/Container'
import VideoPlayer from '@/components/VideoPlayer/VideoPlayer'
import { usePost } from '@/hooks/usePost'

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
    <div className='w-full h-full'>
      <Container className='mb-6'>
        <Breadcrumb onBack={onBack} />
      </Container>
      <VideoPlayer options={videoJsOptions} />
    </div>
  )
}
