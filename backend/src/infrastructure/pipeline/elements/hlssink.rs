use actix::Addr;
use anyhow::Result;
use gio::prelude::*;
use gstreamer::{Element, ElementFactory};
use std::path::Path;
use tokio::runtime::Runtime;
use tracing::*;

use crate::{
    domain::pipeline::ports::HlsSink,
    infrastructure::hls::hls_state_actor::{
        GetPlaylistStream, GetSegmentIndex, HlsStateActor, IncrementSegmentIndex,
    },
};

#[derive(Debug, Clone)]
pub struct HlsSinkImpl {
    element: Element,
}
unsafe impl Send for HlsSinkImpl {}
impl HlsSink for HlsSinkImpl {
    #[instrument]
    fn new(hls_state_actor_addr: Addr<HlsStateActor>) -> Result<Self> {
        // TODO: figure out if we can use hlssink3 for hls on linux
        let element = ElementFactory::make("hlssink2")
            .property("location", "./tmp/segment_%05d.ts")
            .property("playlist-location", "./tmp/event.m3u8")
            // .property("target-duration", 10u32)
            // .property_from_str("playlist-type", "2")
            .property("max-files", 100000u32)
            .property("playlist-length", 0u32)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create hlssink element: {}", e))?;

        debug!("Created hlssink element");

        element.connect("get-fragment-stream", false, {
            let hls_state_actor_addr = hls_state_actor_addr.clone();
            move |args| {
                debug!("get-fragment-stream signal: {:?}", args);

                let path_str = match args[1].get::<String>() {
                    Ok(path) => path,
                    Err(e) => {
                        error!("Failed to get path: {}", e);
                        return None;
                    }
                };
                debug!("original path_str: {}", path_str);

                let path = Path::new(&path_str);
                let parent_path = path.parent().expect("Failed to get parent path");

                let index = match Runtime::new()
                    .unwrap()
                    .block_on(async { hls_state_actor_addr.send(GetSegmentIndex).await })
                {
                    Ok(Ok(index)) => index,
                    Ok(Err(e)) => {
                        error!("Failed to get segment index: {}", e);
                        return None;
                    }
                    Err(e) => {
                        error!("Failed to send message to hls state actor: {}", e);
                        return None;
                    }
                };

                let file_path = parent_path.join(format!("segment_{:05}.ts", index));
                debug!("new file_path: {}", file_path.display());

                let file = gio::File::for_path(file_path);

                // TODO: check if we can reuse the file which was generated before
                if file.query_exists(gio::Cancellable::NONE) {
                    file.delete(gio::Cancellable::NONE)
                        .expect("Failed to delete file");
                }

                let stream = match file.create(
                    gio::FileCreateFlags::REPLACE_DESTINATION,
                    gio::Cancellable::NONE,
                ) {
                    Ok(stream) => {
                        match Runtime::new().unwrap().block_on(async {
                            hls_state_actor_addr.send(IncrementSegmentIndex).await
                        }) {
                            Ok(_) => (),
                            Err(e) => error!("Failed to increment segment index: {}", e),
                        };
                        stream
                    }
                    Err(e) => {
                        error!("Failed to create file: {}", e);
                        return None;
                    }
                };

                Some(stream.to_value())
            }
        });

        // element.connect("get-playlist-stream", false, move |args| {
        //     debug!("get-playlist-stream signal: {:?}", args);

        //     let path_str = match args[1].get::<String>() {
        //         Ok(path) => path,
        //         Err(e) => {
        //             error!("Failed to get path: {}", e);
        //             return None;
        //         }
        //     };
        //     info!("path_str: {}", path_str);
        //     // let stream = PlaylistStream::new(path_str).get_write_stream();
        //     let stream = get_playlist_stream(path_str).get_write_stream();

        //     Some(stream.to_value())
        // });

        element.connect("get-playlist-stream", false, {
            let hls_state_actor_addr = hls_state_actor_addr.clone();
            move |args| {
                debug!("get-playlist-stream signal: {:?}", args);

                let path_str = match args[1].get::<String>() {
                    Ok(path) => path,
                    Err(e) => {
                        error!("Failed to get path: {}", e);
                        return None;
                    }
                };

                let stream = match Runtime::new().unwrap().block_on(async {
                    hls_state_actor_addr.send(GetPlaylistStream(path_str)).await
                }) {
                    Ok(Ok(stream)) => stream.get_write_stream(),
                    Ok(Err(e)) => {
                        error!("Failed to get playlist stream: {}", e);
                        return None;
                    }
                    Err(e) => {
                        error!("Failed to send message to hls state actor: {}", e);
                        return None;
                    }
                };

                Some(stream.to_value())
            }
        });

        Ok(Self { element })
    }

    fn get_element(&self) -> &Element {
        &self.element
    }
}
