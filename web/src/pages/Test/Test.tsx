import { useEffect, useRef } from "react";
import Hls from "hls.js";

export const Test = () => {
  const videoRef = useRef<HTMLVideoElement>(null);

  useEffect(() => {
    const hls = new Hls({
      startPosition: 0,
      // TODO: we will handle the error in the future instead of just keeping retrying
      fragLoadingMaxRetry: 1000,
      fragLoadingRetryDelay: 1000,
      fragLoadingMaxRetryTimeout: 300000,
    })

    hls.loadSource("/hls/playlist.m3u8");
    hls.attachMedia(videoRef.current as HTMLMediaElement);

    hls.on(Hls.Events.MANIFEST_PARSED, () => {
      videoRef.current?.play();
    });

    return () => {
      hls.destroy();
    };
  }, []);

  return (
    <div className="aspect-w-16 aspect-h-9">
      <video slot="media" autoPlay crossOrigin="anonymous" controls ref={videoRef} className="w-full h-full" />
    </div>
  );
};
