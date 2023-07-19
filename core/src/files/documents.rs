use walkdir::WalkDir;
extern crate dirs;
use regex::Regex;

use crate::utils::CommandData;

pub(crate) fn get_documents() -> Result<CommandData<Vec<super::File>>, CommandData<()>> {
    let Some(home_dir) = dirs::home_dir() else {
        return Err(CommandData::err("could not find user's home directory", ()));
    };
    let Some(root_path) = home_dir.as_path().to_str() else {
    return Err(CommandData::err("error getting the document dir",  ()));
    };
    // TODO(@opeolluwa): increase the files supported
    let document_extensions = Regex::new(r"(?i)\.docx$|\.doc$|\.pdf$|\.pptx$").unwrap();
    let root_dir = WalkDir::new(root_path).into_iter();
    let is_document = |entry: walkdir::DirEntry| -> bool {
        document_extensions.is_match(
            entry
                .file_name()
                .to_str()
                .expect("error determining file type"),
        )
    };
    let document_files = root_dir
        .filter_map(|e| e.ok())
        .filter(|e| !super::is_hidden(e))
        .filter(|e| e.file_type().is_file())
        .filter(|e| is_document(e.to_owned()))
        .map(super::File::from)
        .collect();

    Ok(CommandData::ok(
        "successfully fetched document files",
        document_files,
    ))
}
