import 'video.js/dist/video-js.css'
import VideoPlayer from '@/components/VideoPlayer/VideoPlayer'

export const Test = () => {
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

  return (
    <div className='aspect-w-16 aspect-h-9'>
      <VideoPlayer options={videoJsOptions} />
    </div>
  )
}
