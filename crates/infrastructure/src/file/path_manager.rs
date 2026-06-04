use std::path::{Path, PathBuf};

pub struct PathManager {
    assets_dir: PathBuf,
    csv_path: PathBuf
}

impl PathManager {
    pub fn new(base_dir: impl AsRef<Path>, assets_dir_name: &str, csv_name: &str) -> Self {
        let base_dir = base_dir.as_ref().to_path_buf();
        let assets_dir = base_dir.join(assets_dir_name);

        let csv_filename = format!("{}.csv", csv_name);
        let csv_path = assets_dir.join("datasets").join(csv_filename);

        PathManager {
            assets_dir,
            csv_path
        }
    }

    pub fn assets_dir(&self) -> &PathBuf {
        &self.assets_dir
    }

    pub fn csv_path(&self) -> &PathBuf {
        &self.csv_path
    }
}