use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::*;
use ts_rs::TS;
use walkdir::WalkDir;

use super::parsers::TVSerie;
use crate::services::library_parser::processors::process_series;

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct MediaLibrary {
    // TODO: maybe generic type
    pub series: Vec<TVSerie>,
}

#[instrument]
pub fn scan_media_library(root_dir: &Path) -> MediaLibrary {
    debug!("Scanning media library in: {:?}", root_dir);

    let series_dirs: Vec<PathBuf> = WalkDir::new(root_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path().to_path_buf();

            if path.is_dir() {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    debug!("Found {} series directories", series_dirs.len());

    let series_data: Vec<TVSerie> = series_dirs
        .par_iter()
        .map(|series_dir| process_series(series_dir))
        .collect();
    debug!("Processed {} series", series_data.len());

    MediaLibrary {
        series: series_data,
    }
}
