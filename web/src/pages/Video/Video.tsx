import VideoPlayer from '~/components/VideoPlayer/VideoPlayer'

export const Video = () => {
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
    <div className='h-full w-full'>
      <VideoPlayer options={videoJsOptions} />
    </div>
  )
}
