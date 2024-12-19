use rayon::prelude::*;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tracing::*;
use walkdir::WalkDir;

use crate::{
    domain::{library::model::Library, tv_show::model::TvShow},
    infrastructure::{
        event_bus::{domain_event::DomainEvent, event_bus::EventBus, model::GeneralEvent},
        organizer::library::processor::process_series,
    },
};

#[instrument(skip(event_bus))]
pub fn scan_library(root_dir: &Path, event_bus: Arc<EventBus>) -> Library {
    debug!("Scanning library in: {:?}", root_dir);

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

    event_bus.publish(DomainEvent::General(GeneralEvent::TaskProgressUpdated {
        progress: 50.0,
    }));

    let series_data: Vec<TvShow> = series_dirs
        .par_iter()
        .map(|series_dir| process_series(series_dir))
        .collect();
    debug!("Processed {} series", series_data.len());

    Library {
        tv_show: series_data,
    }
}
