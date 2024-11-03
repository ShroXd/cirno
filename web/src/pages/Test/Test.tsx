import { useEffect, useRef } from "react";
import Hls from "hls.js";
import { debounce } from "lodash";
import { useWebSocket } from "../../hooks/useWebSocket";

export const Test = () => {
  const videoRef = useRef<HTMLVideoElement>(null);
  const isRetrying = useRef<boolean>(false);
  const { sendMessage } = useWebSocket();

  useEffect(() => {
    const hls = new Hls({
      startPosition: 0,
      maxBufferLength: 15,
    })

    hls.loadSource("/hls/playlist.m3u8");
    hls.attachMedia(videoRef.current as HTMLMediaElement);

    hls.on(Hls.Events.ERROR, (_event, data) => {
      if (data.details === Hls.ErrorDetails.FRAG_LOAD_ERROR) {
        if (isRetrying.current) return;

        isRetrying.current = true;
        setTimeout(() => {
          hls.startLoad()
          isRetrying.current = false;
        // TODO: do not hard code this retry delay
        }, 5000)
      } else if (data.fatal) {
        switch (data.type) {
          case Hls.ErrorTypes.NETWORK_ERROR:
            console.warn("Network error, attempting to recover...");
            hls.startLoad();
            break;

          case Hls.ErrorTypes.MEDIA_ERROR:
            console.warn("Media error, attempting to recover...");
            hls.recoverMediaError();
            break;

          default:
            console.error("Fatal error encountered, destroying HLS instance.");
            hls.destroy();
            break;
        }
      }
    })

    videoRef.current?.addEventListener("seeking", debounce(() => {
      const currentTimeNs = Math.floor(videoRef.current?.currentTime ?? 0)
      console.log("seeking: ", currentTimeNs)
      sendMessage({
        PipelineAction: {
          Seek: currentTimeNs
        }
      })
    }, 1000))

    videoRef.current?.addEventListener("seeked", debounce(() => {
      console.log("seeked: ", videoRef.current?.currentTime)
    }, 1000))

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
