use anyhow::*;
use rayon::{iter::Either, prelude::*};
use regex::Regex;
use std::path::PathBuf;
use tracing::*;
use walkdir::WalkDir;

#[instrument]
pub fn collect_files(path: &PathBuf) -> Result<Vec<PathBuf>> {
    debug!("Collecting files from path: {}", path.display());

    let files: Vec<PathBuf> = WalkDir::new(path)
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

    Ok(files)
}

#[instrument]
pub fn partition_files(files: &[PathBuf], pattern: &Regex) -> (Vec<PathBuf>, Vec<PathBuf>) {
    debug!("Partitioning files with pattern: {}", pattern);

    files
        .par_iter()
        .filter_map(|file| {
            let file_name = file.file_name()?.to_string_lossy();
            if pattern.is_match(&file_name) {
                Some(Either::Right(file.clone()))
            } else {
                Some(Either::Left(file.clone()))
            }
        })
        .partition_map(|either| either)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::TempDir;

    #[test]
    fn test_collect_files() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let temp_path = temp_dir.path();

        let file1 = temp_path.join("file1.txt");
        let file2 = temp_path.join("file2.txt");
        let subdir = temp_path.join("subdir");
        let file3 = subdir.join("file3.txt");

        File::create(&file1).expect("Failed to create file1.txt");
        File::create(&file2).expect("Failed to create file2.txt");
        fs::create_dir(&subdir).expect("Failed to create subdir");
        File::create(&file3).expect("Failed to create file3.txt");

        let collected_files =
            collect_files(&temp_path.to_path_buf()).expect("Failed to collect files");

        let mut collected_file_names = collected_files
            .into_iter()
            .map(|path| path.strip_prefix(temp_path).unwrap().to_path_buf())
            .collect::<Vec<PathBuf>>();
        collected_file_names.sort();

        let mut expected_file_names = vec![
            PathBuf::from("file1.txt"),
            PathBuf::from("file2.txt"),
            PathBuf::from("subdir/file3.txt"),
        ];
        expected_file_names.sort();

        assert_eq!(collected_file_names, expected_file_names);
    }

    #[test]
    fn test_partition_files_non_matching() {
        let pattern = Regex::new(r"S(\d+)E(\d+)").expect("Failed to create regex");

        let files = vec![
            PathBuf::from("S01E01.mkv"),
            PathBuf::from("S01E02.mkv"),
            PathBuf::from("S01E03.mkv"),
            PathBuf::from("subdir/S01E01.mkv"),
        ];

        let (non_matches, matches) = partition_files(&files, &pattern);

        assert!(non_matches.is_empty());
        assert_eq!(matches.len(), 4);
        assert_eq!(matches, files);
    }

    #[test]
    fn test_partition_files_matching() {
        let pattern = Regex::new(r"S(\d+)E(\d+)").expect("Failed to create regex");

        let files = vec![
            PathBuf::from("S01E01.mkv"),
            PathBuf::from("S01E02.mkv"),
            PathBuf::from("S01E03.mkv"),
            PathBuf::from("subdir/S01E01.mkv"),
            PathBuf::from("tv_serie.nfo"),
        ];

        let (non_matches, matches) = partition_files(&files, &pattern);

        assert_eq!(non_matches.len(), 1);
        assert_eq!(non_matches, vec![PathBuf::from("tv_serie.nfo")]);

        assert_eq!(matches.len(), 4);
        assert_eq!(matches, files[0..4]);
    }
}
