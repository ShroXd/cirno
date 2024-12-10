use ts_rs::TS;

#[derive(Debug, TS, Clone, PartialEq, Eq, Hash)]
#[ts(export)]
pub enum M3u8Tag {
    ExtXVersion,
    ExtXMediaSequence,
    ExtXTargetDuration,
    ExtInf,
}

impl M3u8Tag {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "#EXT-X-VERSION" => Some(Self::ExtXVersion),
            "#EXT-X-MEDIA-SEQUENCE" => Some(Self::ExtXMediaSequence),
            "#EXT-X-TARGETDURATION" => Some(Self::ExtXTargetDuration),
            "#EXTINF" => Some(Self::ExtInf),
            _ => None,
        }
    }
}
