use anyhow::Result;
use gstreamer::Element;
use gstreamer::{prelude::*, Pad};
use std::fmt::Debug;
use tracing::*;

use crate::shared::utils::ElementFactoryTrait;

pub trait Decoder: Send + Sync {
    fn new(factory: &(impl ElementFactoryTrait + Debug)) -> Result<Self>
    where
        Self: Sized;

    fn handle_signal(&mut self, signal: DecodebinSignal, video_sink: Element, audio_sink: Element);
    fn get_element(&self) -> &Element;
}

#[derive(Debug)]
pub enum DecodebinSignal {
    ConnectPadAdded,
}

#[derive(Debug)]
pub struct Decodebin {
    element: Element,
}
unsafe impl Send for Decodebin {}
impl Decoder for Decodebin {
    #[instrument]
    fn new(factory: &(impl ElementFactoryTrait + Debug)) -> Result<Self> {
        let element = factory.make("decodebin3")?;
        debug!("Created decodebin");

        Ok(Self { element })
    }

    #[instrument]
    fn handle_signal(&mut self, signal: DecodebinSignal, video_sink: Element, audio_sink: Element) {
        match signal {
            DecodebinSignal::ConnectPadAdded => {
                let video_sink_clone = video_sink.clone();
                let audio_sink_clone = audio_sink.clone();
                let self_weak = self.element.downgrade();

                self.element.connect_pad_added(move |_dbin, src_pad| {
                    if let Some(self_element) = self_weak.upgrade() {
                        let decodebin = Decodebin {
                            element: self_element,
                        };
                        if let Err(e) = decodebin.pad_added_handler(
                            src_pad,
                            &video_sink_clone,
                            &audio_sink_clone,
                        ) {
                            error!("Failed to handle pad added: {}", e);
                        }
                    }
                });
            }
        }
    }

    fn get_element(&self) -> &Element {
        &self.element
    }
}

impl Decodebin {
    #[instrument]
    fn connect_pad(sink: &Element, src_pad: &Pad, pad_type: &str) -> Result<()> {
        let sink_pad = sink
            .static_pad("sink")
            .ok_or(anyhow::anyhow!("No sink pad"))?;

        if src_pad.is_linked() {
            debug!("{} pad already linked", pad_type);
        } else {
            src_pad
                .link(&sink_pad)
                .map_err(|e| anyhow::anyhow!("Failed to link {} pad: {}", pad_type, e))?;
            debug!("{} pad linked", pad_type);
        }

        Ok(())
    }

    #[instrument]
    fn pad_added_handler(
        &self,
        src_pad: &Pad,
        video_sink: &Element,
        audio_sink: &Element,
    ) -> Result<()> {
        let pad_type = src_pad.name();
        debug!("Prepare to connect pad: {}", pad_type);

        if pad_type.starts_with("video_") {
            Self::connect_pad(video_sink, src_pad, "video")?;
        } else if pad_type.starts_with("audio_") {
            Self::connect_pad(audio_sink, src_pad, "audio")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;

    mock! {
        #[derive(Debug)]
        ElementFactory {}
        impl ElementFactoryTrait for ElementFactory {
            fn make(&self, name: &str) -> Result<Element>;
        }
    }

    #[test]
    fn test_new_success() {
        gstreamer::init().unwrap();
        let mut mock_factory = MockElementFactory::new();
        mock_factory.expect_make().returning(|_| {
            Ok(gstreamer::ElementFactory::make("decodebin")
                .build()
                .unwrap())
        });

        let result = Decodebin::new(&mock_factory);
        assert!(result.is_ok(), "Decodebin::new() should not throw an error");

        let decodebin = result.unwrap();
        assert_eq!(decodebin.element.factory().unwrap().name(), "decodebin");
        assert!(decodebin.element.is::<Element>());
    }

    #[test]
    fn test_new_failure() {
        gstreamer::init().unwrap();
        let mut mock_factory = MockElementFactory::new();
        mock_factory
            .expect_make()
            .returning(|_| Err(anyhow::anyhow!("Failed to make element")));

        let result = Decodebin::new(&mock_factory);
        assert!(result.is_err(), "Decodebin::new() should throw an error");
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to make element",
            "Error message should match"
        );
    }
}
