use crate::command::file::File;
use crate::command::search::search_files;
use crate::utils::CommandData;

static ACCEPTABLE_SUFFIXES: &[&str] = &[
    "3gp", "aa", "aac", "aax", "act", "aiff", "alac", "amr", "ape", "au", "awb", "dss", "dvf",
    "flac", "gsm", "iklax", "ivs", "m4a", "m4b", "m4p", "mmf", "movpkg", "mp3", "mpc", "msv",
    "nmf", "ogg", "oga", "mogg", "opus", "ra", "rm", "raw", "rf64", "sln", "tta", "voc", "vox",
    "wav", "wma", "wv", "webm", "8svx", "cda",
];

// get the audio file from the default audio dir of the OS
// return an instance of the CommandData and vector of the path if any
#[tauri::command]
pub fn fetch_audio_files() -> Result<CommandData<Vec<File>>, CommandData<()>> {
    // if there is an error getting the audio path, fire an error
    let audio_dir = dirs::audio_dir();
    let Some(audio_dir) = audio_dir else{
        return Err(CommandData::err("error getting the audio dir",  ()));
    };

    let entries = search_files("*", &audio_dir)
        .into_iter()
        .filter(|f| ACCEPTABLE_SUFFIXES.contains(&f.file_format.as_str()))
        .collect();

    Ok(CommandData::ok("retrieved all audio files", entries))
}

#[cfg(test)]
mod tests {
    use crate::command::audio::{fetch_audio_files, ACCEPTABLE_SUFFIXES};
    #[test] // see if there are files in the audio directory path
    fn _fetch_audio_files_() {
        let aud_files = fetch_audio_files().ok();
        assert!(aud_files.is_some());

        let aud_files = aud_files.unwrap().data.unwrap();
        for file in aud_files {
            let file_format = file.file_format;
            assert!(ACCEPTABLE_SUFFIXES.contains(&file_format.as_str()));
        }
    }
}
