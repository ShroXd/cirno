use std::path::Path;

pub type FileFilter = Box<dyn Fn(&Path) -> bool + Send + Sync>;

#[derive(Default)]
pub struct FinderOptions {
    pub recursive: bool,
    pub include_hidden: bool,
    pub filters: Option<FileFilter>,
}

impl FinderOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn include_hidden(mut self, include_hidden: bool) -> Self {
        self.include_hidden = include_hidden;
        self
    }

    pub fn filters<F>(mut self, filters: F) -> Self
    where
        F: Fn(&Path) -> bool + Send + Sync + 'static,
    {
        self.filters = Some(Box::new(filters));
        self
    }
}

// TODO: Integrate this
// pub fn by_extension(extension: &[&str]) -> impl Fn(&Path) -> bool + Send + Sync + 'static {
//     let extensions = extension
//         .iter()
//         .map(|e| e.trim_start_matches('.').to_lowercase())
//         .collect::<HashSet<_>>();

//     move |path: &Path| {
//         path.extension()
//             .and_then(|ext| ext.to_str())
//             .map(|ext| extensions.contains(&ext.to_lowercase()))
//             .unwrap_or(false)
//     }
// }

pub fn all_files() -> impl Fn(&Path) -> bool + Send + Sync + 'static {
    move |_| true
}
