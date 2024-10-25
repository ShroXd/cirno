use anyhow::Result;
use gstreamer::prelude::*;
use gstreamer::Element;
use gstreamer::ElementFactory;
use tracing::*;

pub trait Source: Send {
    fn new() -> Result<Self>
    where
        Self: Sized;
    fn set_name(&mut self, name: &str);
    fn set_properties(&mut self, properties: Vec<(&str, &dyn ToValue)>);
    fn get_element(&self) -> &Element;

    fn build(name: &str, properties: Vec<(&str, &dyn ToValue)>) -> Result<Element>
    where
        Self: Sized,
    {
        info!("Created pipeline source {}", name);

        let mut source = Self::new()?;
        source.set_name(name);
        source.set_properties(properties);

        Ok(source.get_element().clone())
    }
}

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
