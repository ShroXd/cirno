use actix::Addr;
use anyhow::*;
use gio::WriteOutputStream;
use std::result::Result::Ok;
use std::sync::Arc;
use std::{collections::HashMap, fs::OpenOptions, io::Write, path::Path};
use tokio::runtime::Runtime;
use tracing::*;

use super::{
    hls_state_actor::{GetPipelineAddr, HlsStateActor},
    model::M3u8Tag,
};
use crate::domain::pipeline::event::PipelineEvent;
use crate::infrastructure::event_bus::domain_event::DomainEvent;
use crate::infrastructure::event_bus::event_bus::EventBus;
use crate::infrastructure::{
    hls::hls_state_actor::SetSegmentDuration, pipeline::actor::QueryDuration,
};

#[derive(Clone)]
pub struct HlsStream {
    path_str: String,
    header: HashMap<M3u8Tag, String>,
    state: Addr<HlsStateActor>,
    initialized: bool,
    event_bus: Arc<EventBus>,
}

impl HlsStream {
    #[instrument(skip(state, event_bus))]
    pub fn new(path_str: String, state: Addr<HlsStateActor>, event_bus: Arc<EventBus>) -> Self {
        Self {
            path_str,
            header: HashMap::new(),
            state,
            initialized: false,
            event_bus,
        }
    }

    #[instrument(skip(self))]
    pub fn get_write_stream(&self) -> WriteOutputStream {
        WriteOutputStream::new(self.clone())
    }

    #[instrument(skip(self, buf))]
    pub fn extract_header(&mut self, buf: &[u8]) -> Result<()> {
        let header_str = std::str::from_utf8(buf).map_err(|e| {
            error!("Failed to convert buffer to string: {}", e);
            e
        })?;

        for line in header_str.lines() {
            let parts = line.splitn(2, ':').collect::<Vec<&str>>();
            if parts.len() == 2 {
                let key = parts[0].to_string();
                if let Some(tag) = M3u8Tag::from_str(&key) {
                    let value = parts[1].to_string();
                    self.header.insert(tag, value);
                }
            }
        }

        if !self.header.is_empty() {
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                match self.create_placeholder_m3u8().await {
                    Ok(_) => info!("Created placeholder m3u8"),
                    Err(e) => error!("Failed to create placeholder m3u8: {}", e),
                }
            });
        }

        Ok(())
    }

    async fn create_placeholder_m3u8(&self) -> Result<()> {
        let duration = self.parse_segment_duration()?;
        self.state
            .send(SetSegmentDuration(duration))
            .await?
            .map_err(|e| {
                error!("Failed to set segment duration: {}", e);
                e
            })?;

        let pipeline_addr = match self.state.send(GetPipelineAddr).await?.map_err(|e| {
            error!("Failed to get pipeline address: {}", e);
            e
        })? {
            Some(addr) => addr,
            None => return Err(anyhow!("Failed to get pipeline address")),
        };

        let file_duration = pipeline_addr.send(QueryDuration).await?;

        let content = self.generate_m3u8_content(&self.header, file_duration, duration)?;

        self.write_m3u8_file(&content)
    }

    fn parse_segment_duration(&self) -> Result<u64> {
        let segment_duration = self
            .header
            .get(&M3u8Tag::ExtInf)
            .ok_or_else(|| anyhow!("Missing EXTINF tag in header"))?;

        let duration_str = segment_duration.trim_end_matches(",");
        let duration = duration_str.parse::<f64>().map_err(|e| {
            error!("Failed to parse segment duration: {}", e);
            e
        })?;
        let duration_nanos = (duration * 1_000_000_000.0) as u64;

        Ok(duration_nanos)
    }

    // TODO: Check duration before seeking
    // async fn query_duration(&self) -> Result<u64> {
    //     let addr = self.state.clone();
    //     let duration = match addr.send(GetPipelineDuration).await {
    //         Ok(Ok(duration)) => duration,
    //         Ok(Err(e)) => {
    //             error!("Failed to get duration: {}", e);
    //             return Err(anyhow!("Failed to get duration: {}", e));
    //         }
    //         Err(e) => {
    //             error!("Failed to send message to hls_state_actor: {}", e);
    //             return Err(anyhow!("Failed to send message to hls_state_actor: {}", e));
    //         }
    //     };

    //     Ok(duration)
    // }

    fn generate_m3u8_content(
        &self,
        header: &HashMap<M3u8Tag, String>,
        file_duration: u64,
        segment_duration: u64,
    ) -> Result<String> {
        let ext_x_version = header
            .get(&M3u8Tag::ExtXVersion)
            .ok_or_else(|| anyhow!("Missing EXT-X-VERSION tag in header"))?;
        let ext_x_media_sequence = header
            .get(&M3u8Tag::ExtXMediaSequence)
            .ok_or_else(|| anyhow!("Missing EXT-X-MEDIA-SEQUENCE tag in header"))?;
        let ext_x_target_duration = header
            .get(&M3u8Tag::ExtXTargetDuration)
            .ok_or_else(|| anyhow!("Missing EXT-X-TARGETDURATION tag in header"))?;
        let segment_duration_str = header
            .get(&M3u8Tag::ExtInf)
            .ok_or_else(|| anyhow!("Missing EXTINF tag in header"))?;

        let file_num_segments = file_duration / segment_duration;

        let m3u8_header = format!(
            "#EXTM3U\n#EXT-X-VERSION:{}\n#EXT-X-MEDIA-SEQUENCE:{}\n#EXT-X-TARGETDURATION:{}\n#EXT-X-PLAYLIST-TYPE:VOD\n#EXT-X-PLAYLIST-LENGTH:{}\n",
            ext_x_version, ext_x_media_sequence, ext_x_target_duration, file_num_segments
        );

        let m3u8_body = (0..file_num_segments)
            .map(|i| format!("#EXTINF:{}\nsegment_{:05}.ts", segment_duration_str, i))
            .collect::<Vec<String>>()
            .join("\n");

        let m3u8_vod_end = "\n#EXT-X-ENDLIST\n";
        let m3u8_content = format!("{}{}{}", m3u8_header, m3u8_body, m3u8_vod_end);

        Ok(m3u8_content)
    }

    fn write_m3u8_file(&self, content: &str) -> Result<()> {
        let path = Path::new(&self.path_str);
        let parent_dir = path
            .parent()
            .expect("Invalid path")
            .to_str()
            .expect("Invalid path");

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("{}/playlist.m3u8", parent_dir))?;

        file.write_all(content.as_bytes())?;

        Ok(())
    }
}

impl Write for HlsStream {
    #[instrument(skip(self, buf))]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let path = Path::new(&self.path_str);

        if !self.initialized {
            if !path.exists() {
                match self.extract_header(buf) {
                    Ok(_) => self.initialized = true,
                    Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
                }
            }
            self.initialized = true;
            if let Err(e) =
                self.event_bus
                    .publish(DomainEvent::Pipeline(PipelineEvent::HlsStreamInitialized {
                        path: self.path_str.clone(),
                    }))
            {
                error!("Failed to publish hls stream initialized event: {}", e);
            }
        }

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(self.path_str.clone())?;

        file.write_all(buf)?;
        debug!("wrote {} bytes to playlist", buf.len());

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
