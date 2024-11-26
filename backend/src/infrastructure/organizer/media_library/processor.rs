use regex::Regex;
use std::path::PathBuf;
use tracing::*;

use crate::{
    domain::{episode::model::Episode, season::model::Season, tv_show::model::TVSerie},
    infrastructure::organizer::media_library::{
        parser::{
            parsers::{parse_episode, parse_season, parse_tv_serie},
            utils::{parse_meta_files, parse_seasons_nfo, parse_tv_series_nfo},
        },
        scanner::utils::{collect_files, partition_files},
        utils::encode_optional_image,
    },
};

pub fn process_series(series_path: &PathBuf) -> TVSerie {
    // Collect all files in the series directory
    let series_files = match collect_files(series_path) {
        Ok(files) => files,
        Err(err) => {
            error!("Error collecting files: {}", err);
            return TVSerie::default();
        }
    };
    debug!("Found {} files", series_files.len());

    // Divide files into meta and episodes
    let episode_pattern = Regex::new(r"S(\d+)E(\d+)").expect("Invalid episode pattern");
    let (meta, episodes) = partition_files(&series_files, &episode_pattern);
    debug!("Divided into {} meta files", meta.len());
    debug!("Divided into {} episodes files", episodes.len());

    // Parse tv series nfo file
    let mut tv_serie = match parse_tv_series_nfo(&series_path, parse_tv_serie) {
        Ok(series) => series,
        Err(err) => {
            error!("Error parsing series nfo file: {}", err);
            return TVSerie::default();
        }
    };

    // Parse tv serie meta files
    parse_meta_files(&meta, &mut tv_serie, encode_optional_image);

    // Parse season nfo files and build seasons
    let mut seasons_map = match parse_seasons_nfo(&meta, parse_season) {
        Ok(seasons) => seasons,
        Err(err) => {
            error!("Error parsing seasons nfo files: {}", err);
            return TVSerie::default();
        }
    };

    // Parse episodes files and build episodes
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
