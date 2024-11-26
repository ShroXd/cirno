use anyhow::*;
use core::result::Result::Ok;
use rayon::prelude::*;
use regex::Regex;
use std::{collections::HashMap, path::PathBuf};
use tracing::*;

use crate::domain::{season::model::Season, tv_show::model::TVSerie};

#[instrument(skip(parse_fn))]
pub fn parse_tv_series_nfo<F>(series_path: &PathBuf, parse_fn: F) -> Result<TVSerie>
where
    F: Fn(&String) -> Result<TVSerie>,
{
    if !series_path.exists() {
        return Err(anyhow!("Series path does not exist"));
    }

    // TODO: use static global file name
    let series_nfo_path = series_path.join("tvshow.nfo");

    let nfo_path = series_nfo_path.to_string_lossy().to_string();
    let mut tv_serie = parse_fn(&nfo_path)?;
    tv_serie.nfo_path = Some(nfo_path);

    Ok(tv_serie)
}

#[instrument(skip(meta_files, encode_fn))]
pub fn parse_meta_files<F>(meta_files: &[PathBuf], tv_serie: &mut TVSerie, encode_fn: F)
where
    F: Fn(&Option<PathBuf>) -> Option<String>,
{
    meta_files.iter().for_each(|file| match file.file_stem() {
        Some(file_stem) => match file_stem.to_string_lossy().as_ref() {
            "poster" => {
                tv_serie.poster_path = encode_fn(&Some(file.clone()));
            }
            "fanart" => {
                tv_serie.fanart_path = encode_fn(&Some(file.clone()));
            }
            _ => {}
        },
        None => return,
    });
}

#[instrument(skip(parse_fn))]
pub fn parse_seasons_nfo<F>(meta_files: &[PathBuf], parse_fn: F) -> Result<HashMap<u8, Season>>
where
    F: Fn(&String) -> Result<Season> + Send + Sync,
{
    let season_pattern = Regex::new(r"S(\d+)/season\.nfo$|season(\d+)\.nfo$").unwrap();

    let seasons_map = meta_files
        .par_iter()
        .filter_map(|season_nfo_file| {
            let season_number = season_pattern
                .captures(&season_nfo_file.to_string_lossy())
                .and_then(|caps| caps.get(1).or(caps.get(2)))
                .and_then(|m| m.as_str().parse::<u8>().ok())?;

            match parse_fn(&season_nfo_file.to_string_lossy().to_string()) {
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

    Ok(seasons_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir_all, write};
    use tempfile::tempdir;

    #[test]
    fn test_parse_series_nfo_with_existing_nfo() {
        let mock_parse = |_: &String| -> Result<TVSerie> { Ok(TVSerie::default()) };

        let temp_dir = tempdir().expect("Failed to create temp dir");
        let series_path = temp_dir.path().join("series");
        create_dir_all(&series_path).expect("Failed to create series dir");
        write(series_path.join("tvshow.nfo"), "").expect("Failed to write nfo file");

        let result =
            parse_tv_series_nfo(&series_path, mock_parse).expect("Failed to parse series nfo");

        assert!(result.nfo_path.is_some());
        assert_eq!(
            result.nfo_path.unwrap(),
            series_path.join("tvshow.nfo").to_string_lossy()
        );
        assert_eq!(result.title, None);
    }

    #[test]
    fn test_parse_series_nfo_without_nfo() {
        let mock_parse = |_: &String| -> Result<TVSerie> { Ok(TVSerie::default()) };

        // Skip creation of nfo file
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let series_path = temp_dir.path().join("series");

        let result = parse_tv_series_nfo(&series_path, mock_parse);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Series path does not exist"
        );
    }

    #[test]
    fn test_parse_seasons_nfo() {
        let mock_parse = |file_path: &String| -> Result<Season> {
            if file_path.contains("invalid") {
                Err(anyhow!("Invalid season nfo file"))
            } else {
                Ok(Season::default())
            }
        };

        let nfo_files = vec![
            PathBuf::from("S01/season.nfo"),
            PathBuf::from("season02.nfo"),
            PathBuf::from("invalid.nfo"),
        ];

        let result = parse_seasons_nfo(&nfo_files, mock_parse).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result.get(&1).unwrap().season_number, Some(1));
        assert_eq!(result.get(&2).unwrap().season_number, Some(2));
    }
}
