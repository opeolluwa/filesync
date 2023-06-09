use crate::utils::{CommandData};
use crate::commands::search::search_files;
use crate::commands::file::File;

// TODO: add more image formats
static ACCEPTABLE_SUFFIXES: &[&str] = &[
    "png", "jpg",
];

fn is_image(file: &File) -> bool {
    let ext = file.file_name.rsplit_once('.');

    match ext {
        Some(ext) => ACCEPTABLE_SUFFIXES.contains(&ext.1),
        None => false,
    }
}

#[tauri::command]
pub fn fetch_image_files() -> Result<CommandData<Vec<File>>, CommandData<()>> {
    let images_dir = dirs::picture_dir();
    let Some(images_dir) = images_dir else{
        return Err(CommandData::err("error getting the images dir",  ()));
    };

    let entries = search_files("*", &images_dir)
        .into_iter()
        .filter(is_image)
        .collect();

    Ok(CommandData::ok("retrieved all images files", entries))
}

#[cfg(test)]
mod tests {
    use crate::commands::image::fetch_image_files;
    #[test] // see if there are files in the image directory path
    fn _fetch_image_files_() {
        let image_files = fetch_image_files().ok();
        assert!(image_files.is_some())
    }
}
