use walkdir::WalkDir;
extern crate dirs;
use regex::Regex;

use crate::utils::CommandData;

pub(crate) fn get_audio_files() -> Result<CommandData<Vec<super::File>>, CommandData<()>> {
    let home_dir = dirs::home_dir().unwrap();
    let Some(root_path) = home_dir.as_path().to_str() else {
    return Err(CommandData::err("error getting the audio dir",  ()));
    };

    // TODO(@opeolluwa): increase the files supported
    let audio_extensions = Regex::new(r"(?i)\.mp3$|\.wav$|\.flac$|\.ogg$").unwrap();
    let mut audio_files = Vec::new();

    for entry in WalkDir::new(root_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !super::is_hidden(e))
        .filter(|e| e.file_type().is_file())
    {
        if let Some(file_name) = entry.file_name().to_str() {
            if audio_extensions.is_match(file_name) {
                let path = entry.path();
                if let Some(path_str) = path.to_str() {
                    audio_files.push(path_str.to_owned());
                }
            }
        }
    }
    Ok(CommandData::ok(
        "successfully fetched audio files",
        audio_files.into_iter().map(super::File::from).collect(),
    ))
    /* audio_files.into_iter().map(File::from) */
}
