use anyhow::*;

pub trait Source: Send + Sync {
    fn get_uri(&self) -> Result<String>;
}

pub trait Decoder: Send + Sync {
    fn supports_format(&self, format: &str) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StreamType {
    Video,
    Audio,
}

pub trait StreamBranch: Send + Sync {
    fn get_branch_type(&self) -> StreamType;
    fn get_caps(&self) -> Option<&str>;
}
