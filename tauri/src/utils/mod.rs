

use std::fs::File;
use std::path::{Path, PathBuf};

// pub mod fs;
pub mod shell;
pub mod system_info;

/// Checks whether given path is a file that can be opened.
/// Returns error if not
pub fn _verify_file_openable(file: &PathBuf) -> Result<(), String> {
    File::open(Path::new(&file))
        .map_err(|err| format!("Error: Cannot open {:?}: {}", file, err))?;
    let is_dir = std::fs::metadata(file)
        .map_err(|err| format!("Error: Cannot query metadata on {:?}: {}", file, err))?
        .is_dir();
    if is_dir {
        return Err(format!("Error: {:?} is a directory.", file));
    }
    Ok(())
}
