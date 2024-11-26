use serde::{Deserialize, Serialize};
use ts_rs::TS;

// TODO: maybe add file info fields here, we can get these info from the nfo file.
#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct Episode {
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub plot: Option<String>,
    pub nfo_path: Option<String>,
    pub video_file_path: String,
    pub subtitle_file_path: Option<String>,
    pub thumb_image_url: Option<String>,
    pub thumb_image: Option<String>,
    pub episode_number: Option<String>,
    pub runtime: Option<String>,
}

impl Default for Episode {
    fn default() -> Self {
        Episode {
            title: None,
            original_title: None,
            plot: None,
            nfo_path: None,
            video_file_path: "".to_string(),
            subtitle_file_path: None,
            thumb_image_url: None,
            thumb_image: None,
            episode_number: None,
            runtime: None,
        }
    }
}

impl Episode {
    pub fn merge(&mut self, other: Episode) {
        if let Some(title) = other.title {
            self.title = Some(title);
        }
        if let Some(original_title) = other.original_title {
            self.original_title = Some(original_title);
        }
        if let Some(plot) = other.plot {
            self.plot = Some(plot);
        }
        if let Some(nfo_path) = other.nfo_path {
            self.nfo_path = Some(nfo_path);
        }
        if !other.video_file_path.is_empty() {
            self.video_file_path = other.video_file_path;
        }
        if let Some(subtitle_file_path) = other.subtitle_file_path {
            self.subtitle_file_path = Some(subtitle_file_path);
        }
        if let Some(thumb_image_url) = other.thumb_image_url {
            self.thumb_image_url = Some(thumb_image_url);
        }
        if let Some(thumb_image) = other.thumb_image {
            self.thumb_image = Some(thumb_image);
        }
        if let Some(episode_number) = other.episode_number {
            self.episode_number = Some(episode_number);
        }
        if let Some(runtime) = other.runtime {
            self.runtime = Some(runtime);
        }
    }
}
