use std::{path::PathBuf};
use walkdir::{WalkDir, DirEntry};
use wildmatch::WildMatch;

use crate::utils::{is_hidden, CommandData};
use crate::commands::file::File;


fn is_wildcard_match(pattern: &str, entry: &DirEntry) -> bool {
    let file_name = entry.file_name();
    let file_name = file_name.to_str().unwrap();
    WildMatch::new(pattern).matches(file_name)
}

#[tauri::command]
pub fn search_home_dir(pattern: &str) -> Result<CommandData<Vec<File>>, CommandData<()>> {
    let home_dir = dirs::home_dir();
    let Some(home_dir) = home_dir else{
        return Err(CommandData::err("error getting the home dir",  ()));
    };

    let entries = search_files(pattern, &home_dir);

    Ok(CommandData::ok("searched all files in home directory", entries))
}

// searches all files in the root directory that matches with the wildcard pattern
// ignores hidden files
// ignores any non-accessible files (e.g. permission errors)
pub fn search_files(pattern: &str, root: &PathBuf) -> Vec<File> {
    let root_dir = WalkDir::new(root).into_iter();
    root_dir
        .filter_map(|e| e.ok())
        .filter(|e| !is_hidden(e))
        .filter(|e| is_wildcard_match(pattern, e))
        .map(File::from)
        .collect()
}
