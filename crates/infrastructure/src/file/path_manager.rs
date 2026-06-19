use std::path::{Path, PathBuf};

/// A utility structure responsible for managing component file paths across the application asset space.
///
/// It constructs and verifies path locations for static data, icons, and dynamic stand images.
pub struct PathManager {
    assets_dir: PathBuf,
    csv_path: PathBuf
}

impl PathManager {
    /// Constructs a new `PathManager` by combining a base directory with specific resource names.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - An object that implements `AsRef<Path>` serving as the root directory of the project.
    /// * `assets_dir_name` - The subfolder name string holding static resources.
    /// * `csv_name` - The base name of the database file without the extension.
    ///
    /// # Returns
    ///
    /// * An initialized `PathManager` with calculated asset and database directory trees.
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

    /// Accesses the root path of the asset directory tree.
    ///
    /// # Returns
    ///
    /// * A shared reference to a `Path` pointing to the application's root asset directory.
    pub fn assets_dir(&self) -> &Path {
        &self.assets_dir
    }

    /// Accesses the designated CSV flat-file path.
    ///
    /// # Returns
    ///
    /// * A shared reference to a `Path` pointing directly to the target CSV database file.
    pub fn csv_path(&self) -> &Path {
        &self.csv_path
    }

    /// Resolves the absolute runtime path for a requested Stand profile image.
    ///
    /// If the specific image file does not exist on disk, it falls back to a default placeholder image.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice referencing the relative path/filename of the stand image.
    ///
    /// # Returns
    ///
    /// * A `PathBuf` container holding the verified image file path or the "unknown.png" fallback location.
    pub fn image_path(&self, path: &str) -> PathBuf {
        let image_path =  self.assets_dir.join("images").join(path);
        if image_path.exists() {
            image_path
        } else {
            self.assets_dir.join("images").join("unknown.png")
        }
    }

    /// Resolves the file path for graphical asset icons.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice referencing the relative path or file descriptor of the icon.
    ///
    /// # Returns
    ///
    /// * A `PathBuf` holding the absolute directory layout path for the icon file.
    pub fn icon_path(&self, path: &str) -> PathBuf {
        self.assets_dir.join("icons").join(path)
    }
}