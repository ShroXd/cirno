use rayon::prelude::*;
use std::path::{Path, PathBuf};
use tracing::*;
use walkdir::WalkDir;

use crate::{
    domain::{media_library::model::MediaLibrary, tv_show::model::TVSerie},
    infrastructure::organizer::media_library::processor::process_series,
};

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
