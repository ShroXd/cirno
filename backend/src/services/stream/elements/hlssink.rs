use anyhow::Result;
use gstreamer::Element;
use gstreamer::ElementFactory;
use tracing::*;

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
            .property("target-duration", 10u32)
            .property("max-files", 100000u32)
            .property("playlist-length", 0u32)
            // .property_from_str("playlist-type", "2")
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create hlssink element: {}", e))?;

        debug!("Created hlssink element");

        Ok(Self { element })
    }

    fn get_element(&self) -> &Element {
        &self.element
    }
}
