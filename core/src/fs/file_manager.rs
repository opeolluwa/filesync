use std::{fs, path::Path};
extern crate dirs;
use path_absolutize::*;

/// read files in a directory

/// get all the files in a directory
/// returns a vector of the file path
pub async fn get_files_in_directory(dir: &Path) -> Result<Vec<String>, String> {
    let paths = fs::read_dir(dir).map_err(|err| err.to_string())?;
    let mut files = Vec::new();
    for path in paths {
        let dir_entry_path = path.unwrap().path();
        let absolutized_path = dir_entry_path.absolutize().unwrap();
        let absolute_path = absolutized_path.to_str().unwrap();
        let file_path = absolute_path.to_string();
        files.push(file_path);
    }

    Ok(files)
}

// fn read_dir() -> io::Result<()> {
//     let home_dir = dirs::home_dir().unwrap();
//     let mut entries = fs::read_dir(home_dir)?
//         .map(|res| res.map(|e| e.path()))
//         .collect::<Result<Vec<_>, io::Error>>()?;

//     entries.sort();
//     println!("{:#?}", entries);

//     Ok(())
// }
