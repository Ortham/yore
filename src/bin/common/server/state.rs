use std::collections::hash_map::DefaultHasher;
use std::ffi::OsStr;
use std::fs::{remove_dir_all, File};
use std::hash::{Hash, Hasher};
use std::io;
use std::path::{Path, PathBuf};

use yore::{load_location_history, GoogleLocationHistory, HistoryError};

use common::photo_paths;

pub struct GuiState {
    root_path: Option<PathBuf>,
    photo_paths: Vec<PathBuf>,
    location_history_path: Option<PathBuf>,
    location_history: GoogleLocationHistory,
    interpolate: bool,
    cache_path: PathBuf,
}

impl GuiState {
    pub fn new(cache_path: &Path) -> GuiState {
        GuiState {
            root_path: None,
            photo_paths: Vec::default(),
            location_history_path: None,
            location_history: GoogleLocationHistory::default(),
            interpolate: false,
            cache_path: cache_path.to_path_buf(),
        }
    }

    pub fn root_path(&self) -> Option<&PathBuf> {
        self.root_path.as_ref()
    }

    pub fn photo_paths(&self) -> &[PathBuf] {
        &self.photo_paths
    }

    pub fn location_history_path(&self) -> Option<&PathBuf> {
        self.location_history_path.as_ref()
    }

    pub fn location_history(&self) -> &GoogleLocationHistory {
        &self.location_history
    }

    pub fn interpolate(&self) -> bool {
        self.interpolate
    }

    pub fn cached_image_path(
        &self,
        original_image_path: &Path,
        width: u32,
        height: u32,
    ) -> PathBuf {
        self.cache_path
            .join(&cached_filename(original_image_path, width, height))
    }

    pub fn search_new_root_path(&mut self, root_path: PathBuf) {
        self.photo_paths = photo_paths(&root_path);
        self.root_path = Some(root_path);
    }

    pub fn load_location_history(&mut self, path: PathBuf) -> Result<(), HistoryError> {
        let file = File::open(&path)?;
        self.location_history = unsafe { load_location_history(&file)? };
        self.location_history_path = Some(path);

        Ok(())
    }

    pub fn set_interpolate(&mut self, interpolate: bool) {
        self.interpolate = interpolate;
    }

    pub fn clear_cache(&self) -> io::Result<()> {
        if self.cache_path.exists() {
            remove_dir_all(&self.cache_path)
        } else {
            Ok(())
        }
    }
}

fn cached_filename(path: &Path, width: u32, height: u32) -> String {
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);

    let extension = path.extension().and_then(OsStr::to_str).unwrap_or(".jpg");

    format!("{}-{}-{}.{}", hasher.finish(), width, height, extension)
}
