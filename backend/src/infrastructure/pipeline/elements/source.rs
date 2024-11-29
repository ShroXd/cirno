use anyhow::Result;
use gstreamer::prelude::*;
use gstreamer::Element;
use gstreamer::ElementFactory;
use tracing::*;

use crate::domain::pipeline::ports::Source;

#[derive(Debug)]
pub struct FileSource {
    pub element: Element,
}
unsafe impl Send for FileSource {}

impl Source for FileSource {
    #[instrument]
    fn new() -> Result<Self> {
        let element = ElementFactory::make("filesrc")
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create filesrc element: {}", e))?;
        debug!("Created filesrc element");

        Ok(Self { element })
    }

    fn set_name(&mut self, name: &str) {
        self.element.set_property("name", name);
        debug!("Set name to {}", name);
    }

    fn set_properties(&mut self, properties: Vec<(&str, &dyn ToValue)>) {
        self.element.set_properties(properties.as_slice());
        for (name, value) in properties {
            debug!("Set property {} to {:?}", name, value.to_value());
        }
    }

    fn get_element(&self) -> &Element {
        &self.element
    }
}
