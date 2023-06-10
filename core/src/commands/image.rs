use crate::commands::file::File;
use crate::commands::search::search_files;
use crate::utils::CommandData;

static ACCEPTABLE_SUFFIXES: &[&str] = &[
    ".jpg", ".jpeg", ".png", ".gif", ".bmp", ".tiff", ".raw", ".svg", ".ai", ".eps", ".psd",
    ".xcf", ".ico", ".webp", ".jxr", ".hdr", ".tif", ".exif", ".pgm", ".ppm", ".pbm", ".pnm",
    ".heic", ".heif", ".dng", ".cr2", ".nef", ".arw", ".orf", ".rw2", ".sr2", ".raf", ".mrw",
    ".pef", ".x3f", ".3fr", ".kdc", ".srw", ".nrw", ".rwz", ".rwl", ".iiq", ".rw1", ".r3d", ".fff",
    ".yuv", ".cin", ".dpx", ".jp2", ".j2k", ".jpf", ".jpx", ".jpm", ".mj2", ".wdp", ".hdp", ".dds",
    ".pvr", ".tga", ".cur", ".icl", ".thm", ".sai", ".ora", ".pdn", ".kra", ".cpt", ".pdd", ".mng",
    ".apng", ".svgz", ".emf", ".wmf",
];

#[tauri::command]
pub fn fetch_images() -> Result<CommandData<Vec<File>>, CommandData<()>> {
    let images_dir = dirs::picture_dir();
    let Some(images_dir) = images_dir else{
        return Err(CommandData::err("error getting the images dir",  ()));
    };

    let entries = search_files("*", &images_dir)
        .into_iter()
        .filter(|f| ACCEPTABLE_SUFFIXES.contains(&f.file_format.as_str()))
        .collect();

    Ok(CommandData::ok("retrieved all images files", entries))
}

#[cfg(test)]
mod tests {
    use crate::commands::image::{fetch_images, ACCEPTABLE_SUFFIXES};
    #[test] // see if there are files in the image directory path
    fn _fetch_image_files_() {
        let images = fetch_images().ok();
        assert!(images.is_some());

        let images = images.unwrap().data.unwrap();
        for file in images {
            let file_format = file.file_format;
            assert!(ACCEPTABLE_SUFFIXES.contains(&file_format.as_str()));
        }
    }
}
