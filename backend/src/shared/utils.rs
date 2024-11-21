use std::path::Path;

/// Checks if the given string represents a valid path on the current operating system
///
/// ## Arguments
/// * `path_str` - The string to validate as a path
///
/// ## Returns
/// * `bool` - true if the string is a valid path, false otherwise
pub fn is_valid_path(path_str: &str) -> bool {
    if path_str.is_empty() {
        return false;
    }

    let path = Path::new(path_str);
    path.is_absolute() && path.exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_path() {
        #[cfg(target_os = "windows")]
        {
            assert!(is_valid_path("C:\\Windows"));
            assert!(!is_valid_path("invalid/path"));
        }

        #[cfg(target_os = "linux")]
        {
            assert!(is_valid_path("/"));
            assert!(!is_valid_path("/this/path/should/not/exist"));
        }

        #[cfg(target_os = "macos")]
        {
            assert!(is_valid_path("/"));
            assert!(!is_valid_path("/this/path/should/not/exist"));
        }

        assert!(!is_valid_path(""));
    }
}
