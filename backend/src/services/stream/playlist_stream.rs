use gio::WriteOutputStream;
use std::{collections::HashMap, io::Write, path::Path};
use tracing::*;
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

#[derive(Debug, Clone)]
pub struct PlaylistStream {
    pub path_str: String,
    header: HashMap<M3u8Tag, String>,
}

impl PlaylistStream {
    pub fn new(path_str: String) -> Self {
        Self {
            path_str,
            header: HashMap::new(),
        }
    }

    pub fn get_write_stream(&self) -> WriteOutputStream {
        WriteOutputStream::new(self.clone())
    }

    /// Note: The current implementation directly parses the buffer without checking if it contains
    /// complete header information. This approach relies on an observed behavior of hlssink2 where
    /// the first buffer is only sent once it contains both complete header information and the first
    /// segment. While this works in practice, it should be noted that this behavior is not explicitly
    /// documented in GStreamer's documentation and may be implementation-specific. Future updates
    /// should verify this behavior through official documentation or GStreamer issue tracking to
    /// ensure long-term stability.
    #[instrument(skip(self, buf))]
    fn extract_header(&mut self, buf: &[u8]) {
        let header_str = std::str::from_utf8(buf).unwrap();
        let header_lines = header_str.split("\n").collect::<Vec<&str>>();

        for line in header_lines {
            let parts = line.split(":").collect::<Vec<&str>>();
            if parts.len() == 2 {
                let key = parts[0].to_string();
                if let Some(tag) = M3u8Tag::from_str(&key) {
                    let value = parts[1].to_string();
                    self.header.insert(tag, value);
                }
            }
        }
        info!("extracted header: {:#?}", self.header);
        self.create_placeholder_m3u8();
    }

    // TODO: consider if we need to split the logic of Write and create_placeholder_m3u8
    #[instrument(skip(self))]
    fn create_placeholder_m3u8(&mut self) {
        let ext_x_version = self
            .header
            .get(&M3u8Tag::ExtXVersion)
            .expect("Missing EXT-X-VERSION tag in header");
        let ext_x_media_sequence = self
            .header
            .get(&M3u8Tag::ExtXMediaSequence)
            .expect("Missing EXT-X-MEDIA-SEQUENCE tag in header");
        let ext_x_target_duration = self
            .header
            .get(&M3u8Tag::ExtXTargetDuration)
            .expect("Missing EXT-X-TARGETDURATION tag in header");
        let segment_duration = self
            .header
            .get(&M3u8Tag::ExtInf)
            .expect("Missing EXTINF tag in header");

        let m3u8_header = format!(
            "#EXTM3U\n#EXT-X-VERSION:{}\n#EXT-X-MEDIA-SEQUENCE:{}\n#EXT-X-TARGETDURATION:{}\n\n",
            ext_x_version, ext_x_media_sequence, ext_x_target_duration
        );
        let m3u8_body = (0..10)
            .map(|i| format!("#EXTINF:{}\nsegment_{:05}.ts", segment_duration, i))
            .collect::<Vec<String>>()
            .join("\n");
        let m3u8_content = format!("{}{}", m3u8_header, m3u8_body);

        let path = std::path::Path::new(&self.path_str);
        let parent_dir = path
            .parent()
            .expect("Invalid path")
            .to_str()
            .expect("Invalid path");

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("{}/playlist.m3u8", parent_dir))
            .unwrap();
        file.write_all(m3u8_content.as_bytes()).unwrap();
    }
}

impl Write for PlaylistStream {
    #[instrument(skip(self, buf))]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let path = Path::new(&self.path_str);
        if self.header.is_empty() && !path.exists() {
            self.extract_header(buf);
        }

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(self.path_str.clone())
            .unwrap();
        file.write_all(buf).unwrap();
        debug!("wrote {} bytes to playlist", buf.len());

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
