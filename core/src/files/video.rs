use crate::utils::CommandData;

pub(crate) fn get_videos() -> Result<CommandData<Vec<super::File>>, CommandData<()>> {
   let Some(home_dir) = dirs::home_dir() else {
        return Err(CommandData::err("could not find user's home directory", ()));
    };
    let Some(root_path) = home_dir.as_path().to_str() else {
    return Err(CommandData::err("error getting the document dir",  ()));
    };
    // TODO(@opeolluwa): increase the files supported
    let image_extensions = Regex::new(r"(?i)\.mp4$|\.avi$|\.mkv$").unwrap();
    let root_dir = WalkDir::new(root_path).into_iter();
    let is_document = |entry: walkdir::DirEntry| -> bool {
        image_extensions.is_match(
            entry
                .file_name()
                .to_str()
                .expect("error determining file type"),
        )
    };
    let images = root_dir
        .filter_map(|e| e.ok())
        .filter(|e| !super::is_hidden(e))
        .filter(|e| e.file_type().is_file())
        .filter(|e| is_document(e.to_owned()))
        .map(super::File::from)
        .collect();

    Ok(CommandData::ok("successfully fetched images", images))
}
