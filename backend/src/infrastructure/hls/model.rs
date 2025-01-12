use ts_rs::TS;

#[derive(Debug, TS, Clone, PartialEq, Eq, Hash)]
#[ts(export)]
pub enum M3u8Tag {
    Version,
    MediaSequence,
    TargetDuration,
    Inf,
}

impl M3u8Tag {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "#EXT-X-VERSION" => Some(Self::Version),
            "#EXT-X-MEDIA-SEQUENCE" => Some(Self::MediaSequence),
            "#EXT-X-TARGETDURATION" => Some(Self::TargetDuration),
            "#EXTINF" => Some(Self::Inf),
            _ => None,
        }
    }
}
