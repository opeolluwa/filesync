use crate::utils::{CommandData};
use crate::commands::search::search_files;
use crate::commands::file::File;

// TODO: add more video formats
static ACCEPTABLE_SUFFIXES: &[&str] = &[
    "mp4", "mkv",
];

fn is_video(file: &File) -> bool {
    let ext = file.file_name.rsplit_once('.');

    match ext {
        Some(ext) => ACCEPTABLE_SUFFIXES.contains(&ext.1),
        None => false,
    }
}

// get the video file from the default video dir of the OS
// return an instance of the CommandData and vector of the path if any
#[tauri::command]
pub fn fetch_video_files() -> Result<CommandData<Vec<File>>, CommandData<()>> {
    // if there is an error getting the video path, fire an error
    let video_dir = dirs::video_dir();
    let Some(video_dir) = video_dir else{
        return Err(CommandData::err("error getting the video dir",  ()));
    };

    let entries = search_files("*", &video_dir)
        .into_iter()
        .filter(is_video)
        .collect();

    Ok(CommandData::ok("retrieved all video files", entries))
}

#[cfg(test)]
mod tests {
    use crate::commands::video::fetch_video_files;
    #[test] // see if there are files in the video directory path
    fn _fetch_video_files_() {
        let vid_files = fetch_video_files().ok();
        assert!(vid_files.is_some())
    }
}
