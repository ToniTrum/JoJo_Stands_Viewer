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
        let csv_path = assets_dir.join("data").join(csv_filename);

        PathManager {
            assets_dir,
            csv_path
        }
    }

    pub fn assets_dir(&self) -> &Path {
        &self.assets_dir
    }

    pub fn csv_path(&self) -> &Path {
        &self.csv_path
    }

    pub fn image_path(&self, path: &str) -> PathBuf {
        let image_path =  self.assets_dir.join("images").join(path);
        if image_path.exists() {
            image_path
        } else {
            self.assets_dir.join("images").join("unknown.png")
        }
    }

    pub fn icon_path(&self, path: &str) -> PathBuf {
        self.assets_dir.join("icons").join(path)
    }
}