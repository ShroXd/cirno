use base64::engine::general_purpose;
use base64::Engine as _;
use regex::Regex;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tracing::*;

#[instrument]
pub fn extract_season_number(path: &Path) -> Option<u8> {
    trace!("Extracting season number from: {:?}", path);

    let dir_name = path.file_name().unwrap().to_string_lossy().to_string();
    trace!("dir_name: {}", dir_name);
    let re = Regex::new(r"S(\d+)").unwrap();
    let caps = re.captures(&dir_name);
    trace!("caps: {:?}", caps);

    if let Some(caps) = caps {
        let season_number = caps.get(1).unwrap().as_str().parse::<u8>().unwrap();
        trace!("season_number: {}", season_number);
        return Some(season_number);
    }

    None
}

#[instrument]
pub fn extract_episode_number(path: &Path) -> Option<u32> {
    trace!("Extracting episode number from: {:?}", path);

    let dir_name = path.file_name().unwrap().to_string_lossy().to_string();
    trace!("dir_name: {}", dir_name);
    let re = Regex::new(r"E(\d+)").unwrap();
    let caps = re.captures(&dir_name);
    trace!("caps: {:?}", caps);

    if let Some(caps) = caps {
        let episode_number = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        trace!("episode_number: {}", episode_number);
        return Some(episode_number);
    }

    None
}

#[instrument]
pub fn encode_optional_image(path: &Option<PathBuf>) -> Option<String> {
    trace!("Encoding optional image: {:?}", path);

    if let Some(image_path) = path {
        if let Ok(image_data) = std::fs::read(image_path) {
            let file_size = image_data.len();
            trace!(
                "Image data length: {} bytes ({:.2} MB)",
                file_size,
                file_size as f64 / 1_048_576.0
            );

            let base64_string = format!(
                "data:image/png;base64,{}",
                general_purpose::STANDARD.encode(image_data)
            );
            return Some(base64_string);
        }
    }

    None
}

#[instrument]
pub fn find_file_with_prefix(dir: &Path, prefix: &str) -> Option<PathBuf> {
    trace!("Searching for file with prefix: {}", prefix);
    fs::read_dir(dir)
        .ok()?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.file_stem()?.to_string_lossy().starts_with(prefix) {
                Some(path)
            } else {
                None
            }
        })
        .next()
}

#[instrument]
pub fn is_season_dir(path: &Path) -> bool {
    trace!("Checking if path is a season directory: {:?}", path);

    let dir_name = path.file_name().unwrap().to_string_lossy().to_string();
    trace!("dir_name: {}", dir_name);
    dir_name.contains("season") || dir_name.contains("s") || dir_name.contains("S")
}

#[instrument]
pub fn is_video_file(path: &Path) -> bool {
    trace!("Checking if path is a video file: {:?}", path);

    match path.extension() {
        Some(ext) => {
            let ext = ext.to_string_lossy().to_lowercase();
            trace!("Extension: {}", ext);
            ext == "mp4" || ext == "mkv" || ext == "avi" || ext == "mov" || ext == "wmv"
        }
        None => {
            debug!("No extension found");
            false
        }
    }
}

#[instrument]
pub fn find_associated_file(dir: &Path, base_name: &str, extensions: &[&str]) -> Option<PathBuf> {
    trace!("Searching for associated file in {:?}", dir);

    for ext in extensions {
        let file_path = dir.join(format!("{base_name}.{ext}"));
        trace!("Checking file: {:?}", file_path);

        if file_path.exists() {
            return Some(file_path);
        }
    }

    None
}
