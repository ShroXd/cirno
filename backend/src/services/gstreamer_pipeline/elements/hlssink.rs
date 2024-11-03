use anyhow::Result;
use gio::prelude::*;
use gstreamer::{Element, ElementFactory};
use tracing::*;

use crate::init::system_initializer::get_playlist_stream;

pub trait HlsSink: Send {
    fn new() -> Result<Self>
    where
        Self: Sized;
    fn get_element(&self) -> &Element;
}

#[derive(Debug, Clone)]
pub struct HlsSinkImpl {
    element: Element,
}
unsafe impl Send for HlsSinkImpl {}
impl HlsSink for HlsSinkImpl {
    #[instrument]
    fn new() -> Result<Self> {
        // TODO: figure out if we can use hlssink3 for hls on linux
        let element = ElementFactory::make("hlssink2")
            .property("location", "./tmp/segment_%05d.ts")
            .property("playlist-location", "./tmp/event.m3u8")
            // .property("target-duration", 10u32)
            .property("max-files", 100000u32)
            .property("playlist-length", 0u32)
            // .property_from_str("playlist-type", "2")
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create hlssink element: {}", e))?;

        debug!("Created hlssink element");

        // element.connect("get-fragment-stream", false, {
        //     move |args| {
        //         debug!("get-fragment-stream signal: {:?}", args);

        //         let path_str = match args[1].get::<String>() {
        //             Ok(path) => path,
        //             Err(e) => {
        //                 error!("Failed to get path: {}", e);
        //                 return None;
        //             }
        //         };
        //         debug!("path_str: {}", path_str);
        //         let path = Path::new(&path_str);
        //         let file = gio::File::for_path(path);

        //         let stream = match file.create(gio::FileCreateFlags::NONE, gio::Cancellable::NONE) {
        //             Ok(stream) => stream,
        //             Err(e) => {
        //                 error!("Failed to create file: {}", e);
        //                 return None;
        //             }
        //         };

        //         Some(stream.to_value())
        //     }
        // });

        element.connect("get-playlist-stream", false, move |args| {
            debug!("get-playlist-stream signal: {:?}", args);

            let path_str = match args[1].get::<String>() {
                Ok(path) => path,
                Err(e) => {
                    error!("Failed to get path: {}", e);
                    return None;
                }
            };
            info!("path_str: {}", path_str);
            // let stream = PlaylistStream::new(path_str).get_write_stream();
            let stream = get_playlist_stream(path_str).get_write_stream();

            Some(stream.to_value())
        });

        Ok(Self { element })
    }

    fn get_element(&self) -> &Element {
        &self.element
    }
}
