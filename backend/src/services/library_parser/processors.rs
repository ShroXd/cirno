use rayon::iter::Either;
use rayon::prelude::*;
use regex::Regex;
use std::{collections::HashMap, path::PathBuf};
use tracing::*;
use walkdir::WalkDir;

use crate::services::library_parser::parsers::parse_episode;

use super::{
    parsers::{parse_season, parse_tv_serie, Episode, Season, TVSerie},
    utils::encode_optional_image,
};

pub fn process_series(series_path: &PathBuf) -> TVSerie {
    let series_files: Vec<PathBuf> = WalkDir::new(series_path)
        .min_depth(1)
        .max_depth(10)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path().to_path_buf();
            if path.is_file() {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    let episode_pattern = Regex::new(r"S(\d+)E(\d+)").unwrap();

    // Divide files into meta and episodes
    let (meta, episodes): (Vec<PathBuf>, Vec<PathBuf>) = series_files
        .into_par_iter()
        .filter_map(|series_dir| {
            let file_name = series_dir.file_name()?.to_string_lossy().to_string();
            if episode_pattern.is_match(&file_name) {
                Some(Either::Right(series_dir))
            } else {
                Some(Either::Left(series_dir))
            }
        })
        .partition_map(|either| either);

    debug!("Found {} meta directories", meta.len());
    debug!("Found {} episodes directories", episodes.len());

    // Parse series nfo file
    let series_nfo_path = series_path.join("tvshow.nfo");
    let mut tv_serie = TVSerie::default();
    if series_nfo_path.exists() {
        let nfo_path = series_nfo_path.to_string_lossy().to_string();
        let series_nfo = parse_tv_serie(&nfo_path);

        tv_serie = series_nfo.unwrap();
        tv_serie.nfo_path = Some(nfo_path);
    }

    // Parse meta files
    meta.iter().for_each(|file| match file.file_stem() {
        Some(file_stem) => match file_stem.to_string_lossy().as_ref() {
            "poster" => {
                tv_serie.poster_path = encode_optional_image(&Some(file.clone()));
            }
            "fanart" => {
                tv_serie.fanart_path = encode_optional_image(&Some(file.clone()));
            }
            _ => {}
        },
        None => return,
    });

    // Parse season nfo files and build seasons
    let season_pattern = Regex::new(r"S(\d+)/season\.nfo$|season(\d+)\.nfo$").unwrap();
    let mut seasons_map: HashMap<u8, Season> = meta
        .par_iter()
        .filter(|file| season_pattern.is_match(file.to_string_lossy().as_ref()))
        .filter_map(|season_nfo_file| {
            let season_number = season_pattern
                .captures(&season_nfo_file.to_string_lossy())
                .and_then(|caps| caps.get(1).or(caps.get(2)))
                .and_then(|m| m.as_str().parse::<u8>().ok())?;

            match parse_season(&season_nfo_file.to_string_lossy().to_string()) {
                Ok(mut season) => {
                    season.season_number = Some(season_number);
                    Some((season_number, season))
                }
                Err(err) => {
                    error!("Error parsing season nfo file: {}", err);
                    None
                }
            }
        })
        .collect();

    let episode_pattern = Regex::new(r"S(\d+)E(\d+)").unwrap();
    episodes.iter().for_each(|episode_dir| {
        let file_name = match episode_dir.file_stem() {
            Some(file_name) => file_name.to_string_lossy().to_string(),
            None => return,
        };

        let (season_number, episode_number) = match episode_pattern.captures(&file_name) {
            Some(caps) => {
                // TODO: maybe we need to process the error here
                let season_number = caps
                    .get(1)
                    .and_then(|m| m.as_str().parse::<u8>().ok())
                    .unwrap_or_default();
                let episode_number = caps
                    .get(2)
                    .and_then(|m| m.as_str().parse::<u8>().ok())
                    .unwrap_or_default();
                (season_number, episode_number)
            }
            None => {
                warn!("Error parsing episode file: {}", file_name);
                return;
            }
        };

        let season_map_key = season_number;

        if !seasons_map.contains_key(&season_map_key) {
            // TODO: maybe we dont need this
            debug!("Adding season: {}", season_number);
            seasons_map.insert(season_map_key, Season::default());
        }

        let season = seasons_map.get_mut(&season_map_key).unwrap();
        if !season.episodes.contains_key(&episode_number) {
            season.episodes.insert(episode_number, Episode::default());
        }

        let extension = match episode_dir.extension() {
            Some(ext) => ext.to_string_lossy().to_lowercase(),
            None => "".to_string(),
        };

        match extension.as_ref() {
            "mp4" | "mkv" | "avi" | "mov" | "wmv" => {
                let episode = season.episodes.get_mut(&episode_number).unwrap();
                episode.video_file_path = episode_dir.to_string_lossy().to_string();
            }
            "jpg" | "jpeg" | "png" | "webp" => {
                let episode = season.episodes.get_mut(&episode_number).unwrap();
                // TODO: what is the difference between thumb_image and thumbnail_image? fuck
                episode.thumb_image = encode_optional_image(&Some(episode_dir.clone()));
            }
            "srt" | "ass" | "ssa" => {
                // TODO: add subtitle to episode
            }
            "nfo" => {
                let existing_episode = season.episodes.get_mut(&episode_number).unwrap();
                let episode_from_nfo =
                    match parse_episode(&episode_dir.to_string_lossy().to_string()) {
                        Ok(episode) => episode,
                        Err(err) => {
                            error!("Error parsing episode nfo file: {}", err);
                            return;
                        }
                    };
                existing_episode.merge(episode_from_nfo);
            }
            _ => {}
        }
    });

    tv_serie.seasons = seasons_map;

    tv_serie
}
