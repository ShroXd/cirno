use anyhow::Result;
use gstreamer::prelude::*;
use gstreamer::ElementFactory;
use gstreamer::{Caps, Element};
use std::fmt::Debug;
use tracing::{debug, instrument};

use crate::{domain::pipeline::ports::StreamBranch, shared::utils::ElementFactoryTrait};

#[derive(Debug)]
pub struct VideoBranch {
    queue: Element,
    converter: Element,
    encoder: Element,
    parser: Element,
}
unsafe impl Send for VideoBranch {}
impl StreamBranch for VideoBranch {
    #[instrument]
    fn new(factory: &(impl ElementFactoryTrait + Debug)) -> Result<Self> {
        let queue = factory.make("queue")?;
        let converter = factory.make("videoconvert")?;
        let encoder = generate_encoder()?;

        let parser = factory.make("h264parse")?;

        debug!("VideoBranch created with elements: videoconvert, x264enc, h264parse");

        Ok(Self {
            queue,
            converter,
            encoder,
            parser,
        })
    }

    fn get_entry(&self) -> Element {
        self.queue.clone()
    }

    fn get_elements(&self) -> Vec<&Element> {
        vec![&self.queue, &self.converter, &self.encoder, &self.parser]
    }
}

#[cfg(target_os = "linux")]
// TODO: use hardware encoder for linux
fn generate_encoder() -> Result<Element> {
    ElementFactory::make("x264enc")
        .property_from_str("speed-preset", "superfast")
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to create x264enc element: {}", e))
}

#[cfg(target_os = "macos")]
fn generate_encoder() -> Result<Element> {
    let encoder = ElementFactory::make("vtenc_h264")
        .property("max-keyframe-interval", 30)
        .property("max-keyframe-interval-duration", 2_000_000_000u64)
        .property("realtime", true)
        .property("allow-frame-reordering", false)
        .build()?;
    encoder.set_property("bitrate", 8000u32);

    Ok(encoder)
}

#[derive(Debug)]
pub struct AudioBranch {
    queue: Element,
    converter: Element,
    resampler: Element,
    capsfilter: Element,
    encoder: Element,
    // payloader: Element,
}
unsafe impl Send for AudioBranch {}
impl StreamBranch for AudioBranch {
    #[instrument]
    fn new(factory: &(impl ElementFactoryTrait + Debug)) -> Result<Self> {
        let queue = factory.make("queue")?;
        let converter = factory.make("audioconvert")?;
        let resampler = factory.make("audioresample")?;
        let capsfilter = factory.make("capsfilter")?;

        // TODO: maybe we have better way to set capsfilter
        let cap = Caps::builder("audio/x-raw");
        capsfilter.set_property("caps", cap.build());

        // let encoder = factory.make("opusenc")?;
        let encoder = factory.make("avenc_aac")?;
        // let payloader = factory.make("rtpopuspay")?;

        debug!("AudioBranch created with elements: audioconvert, audioresample, capsfilter, opusenc, rtpopuspay");

        Ok(Self {
            queue,
            converter,
            resampler,
            capsfilter,
            encoder,
            // payloader,
        })
    }

    fn get_entry(&self) -> Element {
        self.queue.clone()
    }

    fn get_elements(&self) -> Vec<&Element> {
        vec![
            &self.queue,
            &self.converter,
            &self.resampler,
            &self.capsfilter,
            &self.encoder,
            // &self.payloader,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gstreamer::ElementFactory;
    use mockall::*;

    mock! {
        #[derive(Debug)]
        ElementFactory {}
        impl ElementFactoryTrait for ElementFactory {
            fn make(&self, name: &str) -> Result<Element>;
        }
    }

    #[test]
    fn test_video_branch_new_success() {
        gstreamer::init().unwrap();

        let mut mock_factory = MockElementFactory::new();
        mock_factory
            .expect_make()
            .returning(|name| Ok(ElementFactory::make(name).build().unwrap()));

        let result = VideoBranch::new(&mock_factory);
        assert!(result.is_ok(), "VideoBranch::new should not throw an error");

        let branch = result.unwrap();
        let elements = branch.get_elements();
        assert_eq!(elements.len(), 4, "VideoBranch should have 5 elements");
    }

    #[test]
    fn test_video_branch_new_failure() {
        gstreamer::init().unwrap();

        let mut mock_factory = MockElementFactory::new();
        mock_factory
            .expect_make()
            .returning(|_| Err(anyhow::anyhow!("Failed to make element")));

        let result = VideoBranch::new(&mock_factory);
        assert!(result.is_err(), "VideoBranch::new should throw an error");
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to make element",
            "Error message should match"
        );
    }

    #[test]
    fn test_audio_branch_new_success() {
        gstreamer::init().unwrap();

        let mut mock_factory = MockElementFactory::new();
        mock_factory
            .expect_make()
            .returning(|name| Ok(ElementFactory::make(name).build().unwrap()));

        let result = AudioBranch::new(&mock_factory);
        assert!(result.is_ok(), "AudioBranch::new should not throw an error");

        let branch = result.unwrap();
        let elements = branch.get_elements();
        assert_eq!(elements.len(), 5, "AudioBranch should have 6 elements");
    }

    #[test]
    fn test_audio_branch_new_failure() {
        gstreamer::init().unwrap();

        let mut mock_factory = MockElementFactory::new();
        mock_factory
            .expect_make()
            .returning(|_| Err(anyhow::anyhow!("Failed to make element")));

        let result = AudioBranch::new(&mock_factory);
        assert!(result.is_err(), "AudioBranch::new should throw an error");
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to make element",
            "Error message should match"
        );
    }
}
