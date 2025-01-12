use gio::WriteOutputStream;
use std::{collections::HashMap, io::Write, path::Path};
use tokio::runtime::Runtime;
use tracing::*;

use super::model::M3u8Tag;
use crate::{
    infrastructure::video_pipeline::actor::QueryDuration,
    init::app_state::{get_pipeline_addr, set_pipeline_duration, set_pipeline_segment_duration},
};

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

    #[instrument(skip(self))]
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

        let header_clone = self.header.clone();
        let path_str_clone = self.path_str.clone();

        // TODO: find a better way to do this
        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            create_placeholder_m3u8(header_clone, path_str_clone).await;
        });
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
            .truncate(false)
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

// TODO: consider if we need to split the logic of Write and create_placeholder_m3u8
// TODO: refactor this shit, do not use such ugly code to calculate the number of segments
#[instrument]
async fn create_placeholder_m3u8(header: HashMap<M3u8Tag, String>, path_str: String) {
    let addr = get_pipeline_addr();

    let ext_x_version = header
        .get(&M3u8Tag::Version)
        .expect("Missing EXT-X-VERSION tag in header");
    let ext_x_media_sequence = header
        .get(&M3u8Tag::MediaSequence)
        .expect("Missing EXT-X-MEDIA-SEQUENCE tag in header");
    let ext_x_target_duration = header
        .get(&M3u8Tag::TargetDuration)
        .expect("Missing EXT-X-TARGETDURATION tag in header");
    let segment_duration = header
        .get(&M3u8Tag::Inf)
        .expect("Missing EXTINF tag in header");

    info!("segment_duration: {:?}", segment_duration);
    // TODO: do this during the extraction of the header
    let trimmed_segment_duration = segment_duration.trim_end_matches(",");
    match trimmed_segment_duration.parse::<f64>() {
        Ok(duration) => {
            let duration_nanos = (duration * 1_000_000_000.0) as u64;
            set_pipeline_segment_duration(duration_nanos);
        }
        Err(e) => error!("Failed to parse segment duration: {}", e),
    }

    let m3u8_header = format!(
        "#EXTM3U\n#EXT-X-VERSION:{}\n#EXT-X-MEDIA-SEQUENCE:{}\n#EXT-X-TARGETDURATION:{}\n\n",
        ext_x_version, ext_x_media_sequence, ext_x_target_duration
    );

    let raw_duration_str = &segment_duration.to_string();
    let duration_secs_str = &raw_duration_str.trim_end_matches(",");
    info!("duration_secs_str: {:?}", duration_secs_str);
    let duration_secs = match duration_secs_str.parse::<f64>() {
        Ok(duration) => duration,
        Err(e) => {
            error!("Failed to parse segment duration: {}", e);
            return;
        }
    };
    let duration_nanos = (duration_secs * 1_000_000_000.0) as u64;
    let file_duration = match addr.send(QueryDuration).await {
        Ok(duration) => duration,
        Err(e) => {
            error!("Failed to get duration: {}", e);
            return;
        }
    };
    set_pipeline_duration(file_duration);

    let file_num_segments = file_duration / duration_nanos;
    info!("file_num_segments: {:?}", file_num_segments);

    let m3u8_body = (0..file_num_segments)
        .map(|i| format!("#EXTINF:{}\nsegment_{:05}.ts", segment_duration, i))
        .collect::<Vec<String>>()
        .join("\n");
    let m3u8_vod_end = "\n#EXT-X-ENDLIST\n";
    let m3u8_content = format!("{}{}{}", m3u8_header, m3u8_body, m3u8_vod_end);

    let path = std::path::Path::new(&path_str);
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
