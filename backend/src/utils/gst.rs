use anyhow::Result;
use gstreamer::{Element, ElementFactory as GstElementFactory};

pub trait ElementFactoryTrait {
    fn make(&self, element_name: &str) -> Result<Element>;
}

#[derive(Debug, Clone)]
pub struct ElementFactory;
impl ElementFactoryTrait for ElementFactory {
    fn make(&self, element_name: &str) -> Result<Element> {
        let element = GstElementFactory::make(element_name)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create element: {}", e))?;

        Ok(element)
    }
}
