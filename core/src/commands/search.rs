#![allow(unused_imports)]
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};
use wildmatch::WildMatch;

use crate::commands::file::File;
use crate::utils::{is_hidden, CommandData};
use assert_fs::prelude::*;

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

    Ok(CommandData::ok(
        "searched all files in home directory",
        entries,
    ))
}

// searches all files in the root directory that matches with the wildcard pattern
// ignores hidden files
// ignores any non-accessible files (e.g. permission errors)
pub fn search_files(pattern: &str, root: &PathBuf) -> Vec<File> {
    let root_dir = WalkDir::new(root).into_iter();
    root_dir
        .filter_map(|e| e.ok())
        .filter(|e| !is_hidden(e))
        .filter(|e| e.file_type().is_file())
        .filter(|e| is_wildcard_match(pattern, e))
        .map(File::from)
        .collect()
}

#[test]
fn test_search_files_recursive() {
    let temp = assert_fs::TempDir::new().unwrap();
    let root = temp.path().to_path_buf();

    let file1 = temp.child("file1.txt");
    file1.touch().unwrap();

    let file2 = temp.child("file2.png");
    file2.touch().unwrap();

    let a = temp.child("a.txt");
    a.touch().unwrap();

    let dir1 = temp.child("dir1");
    dir1.create_dir_all().unwrap();

    let file3 = dir1.child("file3.mp4");
    file3.touch().unwrap();

    let results = search_files("*", &root);
    assert_eq!(results.len(), 4);
}

#[test]
fn test_search_files_hidden_not_included() {
    let temp = assert_fs::TempDir::new().unwrap();
    let root = temp.path().to_path_buf();

    let file1 = temp.child("file1.txt");
    file1.touch().unwrap();

    let file2 = temp.child(".file2.png");
    file2.touch().unwrap();

    let results = search_files("*", &root);
    assert_eq!(results.len(), 1);

    for file in results {
        assert_ne!(file.file_name, ".file2.png");
    }
}

#[test]
fn test_search_files_with_pattern() {
    let temp = assert_fs::TempDir::new().unwrap();
    let root = temp.path().to_path_buf();

    let file1 = temp.child("file1.txt");
    file1.touch().unwrap();

    let file2 = temp.child("file2.png");
    file2.touch().unwrap();

    let results = search_files("*.txt", &root);
    assert_eq!(results.len(), 1);

    for file in results {
        assert_eq!(file.file_name, "file1.txt");
    }
}

#[test]
fn test_search_files_with_pattern_recursive() {
    let temp = assert_fs::TempDir::new().unwrap();
    let root = temp.path().to_path_buf();

    let file1 = temp.child("file1.txt");
    file1.touch().unwrap();

    let file2 = temp.child("file2.png");
    file2.touch().unwrap();

    let dir1 = temp.child("dir1");
    dir1.create_dir_all().unwrap();

    let file3 = dir1.child("file3.mp4");
    file3.touch().unwrap();

    let results = search_files("*.txt", &root);
    assert_eq!(results.len(), 1);

    for file in results {
        assert_eq!(file.file_name, "file1.txt");
    }
}

#[test]
fn test_search_files_empty() {
    let temp = assert_fs::TempDir::new().unwrap();
    let root = temp.path().to_path_buf();

    let results = search_files("*", &root);
    assert_eq!(results.len(), 0);
}

#[test]
fn test_search_files_nonexistent_dir() {
    let temp = assert_fs::TempDir::new().unwrap();
    let root = temp.path().to_path_buf();
    temp.close().unwrap();

    let results = search_files("*", &root);
    assert_eq!(results.len(), 0);
}

#[test]
fn test_search_files_with_nonascii_filename() {
    let temp = assert_fs::TempDir::new().unwrap();
    let root = temp.path().to_path_buf();

    let file1 = temp.child("file1.txt");
    file1.touch().unwrap();

    let file2 = temp.child("file2.png");
    file2.touch().unwrap();

    let file3 = temp.child("🦀🦀🦀.txt");
    file3.touch().unwrap();

    let results = search_files("*.txt", &root);
    assert_eq!(results.len(), 2);

    for file in results {
        assert!(file.file_name == "file1.txt" || file.file_name == "🦀🦀🦀.txt");
    }
}
