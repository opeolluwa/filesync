use walkdir::WalkDir;
extern crate dirs;
use regex::Regex;

use crate::utils::CommandData;

pub(crate) fn get_audio_files() -> Result<CommandData<Vec<super::File>>, CommandData<()>> {
    let Some(home_dir) = dirs::home_dir() else {
        return Err(CommandData::err("could not find user's home directory", ()));
    };
    let Some(root_path) = home_dir.as_path().to_str() else {
    return Err(CommandData::err("error getting the audio dir",  ()));
    };
    // TODO(@opeolluwa): increase the files supported
    let audio_extensions = Regex::new(r"(?i)\.mp3$|\.wav$|\.flac$|\.ogg$").unwrap();
    let root_dir = WalkDir::new(root_path).into_iter();
    let is_audio = |entry: walkdir::DirEntry| -> bool {
        audio_extensions.is_match(
            entry
                .file_name()
                .to_str()
                .expect("error determining file type"),
        )
    };
    let audio_files = root_dir
        .filter_map(|e| e.ok())
        .filter(|e| !super::is_hidden(e))
        .filter(|e| e.file_type().is_file())
        .filter(|e| is_audio(e.to_owned()))
        .map(super::File::from)
        .collect();

    Ok(CommandData::ok(
        "successfully fetched audio files",
        audio_files,
    ))
}
