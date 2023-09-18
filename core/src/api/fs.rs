use std::net::Ipv4Addr;

use serde_json::{json, Value};

use crate::fs::file::File;
use crate::fs::search::search_files;
use crate::utils::CommandData;
use crate::wifi::ip_manager;
use tokio::io::AsyncReadExt;

pub mod document {
    use super::*;

    /// filter file path for documents
    static ACCEPTABLE_SUFFIXES: &[&str] = &[
        "ppt", "pot", "pps", "pptx", "pptm", "potx", "potm", "ppam", "ppsx", "ppsm", "sldx",
        "sldm", "odp", "fodp", "otp", "doc", "dot", "docx", "docm", "dotx", "dotm", "docb", "odt",
        "fodt", "ott", "ots", "xls", "xlt", "xlm", "xlsx", "xlsm", "xltx", "xltm", "xla", "xlam",
        "ods", "fods", "xml", "xslt", "html", "xhtml", "htm", "txt", "rtf", "c", "h", "cpp", "hpp",
        "cxx", "hxx", "java", "js", "rb", "py", "cs", "m", "sh", "php", "css", "go", "ps", "rs",
        "pdf",
    ];

    // get the documents from the default documents dir of the OS
    // return an instance of the CommandData and vector of the path if any
    #[tauri::command]
    pub fn fetch_documents() -> Result<CommandData<Vec<File>>, CommandData<()>> {
        // if there is an error getting the documents path, fire an error
        let document_dir = dirs::document_dir();
        let Some(document_dir) = document_dir else {
            return Err(CommandData::err("error getting the documents dir", ()));
        };

        let entries = search_files("*", &document_dir)
            .into_iter()
            .filter(|f| ACCEPTABLE_SUFFIXES.contains(&f.file_format.as_str()))
            .collect();

        Ok(CommandData::ok("retrieved all documents", entries))
    }

    #[cfg(test)]
    mod tests {
        use super::{fetch_documents, ACCEPTABLE_SUFFIXES};
        #[test] // see if there are files in the documents directory path
        fn _fetch_documents_() {
            let docs = fetch_documents().ok();
            assert!(docs.is_some());

            let files = docs.unwrap().data.unwrap();
            for file in files {
                let file_format = file.file_format;
                assert!(ACCEPTABLE_SUFFIXES.contains(&file_format.as_str()));
            }
        }
    }
}

pub mod image {
    use super::*;
    static ACCEPTABLE_SUFFIXES: &[&str] = &[
        "jpg", "jpeg", "png", "gif", "bmp", "tiff", "raw", "svg", "ai", "eps", "psd", "xcf", "ico",
        "webp", "jxr", "hdr", "tif", "exif", "pgm", "ppm", "pbm", "pnm", "heic", "heif", "dng",
        "cr2", "nef", "arw", "orf", "rw2", "sr2", "raf", "mrw", "pef", "x3f", "3fr", "kdc", "srw",
        "nrw", "rwz", "rwl", "iiq", "rw1", "r3d", "fff", "yuv", "cin", "dpx", "jp2", "j2k", "jpf",
        "jpx", "jpm", "mj2", "wdp", "hdp", "dds", "pvr", "tga", "cur", "icl", "thm", "sai", "ora",
        "pdn", "kra", "cpt", "pdd", "mng", "apng", "svgz", "emf", "wmf",
    ];

    #[tauri::command]
    pub fn fetch_images() -> Result<CommandData<Vec<File>>, CommandData<()>> {
        let images_dir = dirs::picture_dir();
        let Some(images_dir) = images_dir else {
            return Err(CommandData::err("error getting the images dir", ()));
        };

        let entries = search_files("*", &images_dir)
            .into_iter()
            .filter(|f| ACCEPTABLE_SUFFIXES.contains(&f.file_format.as_str()))
            .collect();

        Ok(CommandData::ok("retrieved all images files", entries))
    }

    #[cfg(test)]
    mod tests {
        use super::{fetch_images, ACCEPTABLE_SUFFIXES};
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
}

/* the audion module is responsible for parsing audio relater commands */
pub mod audio {
    use super::*;
    static ACCEPTABLE_SUFFIXES: &[&str] = &[
        "3gp", "aa", "aac", "aax", "act", "aiff", "alac", "amr", "ape", "au", "awb", "dss", "dvf",
        "flac", "gsm", "iklax", "ivs", "m4a", "m4b", "m4p", "mmf", "movpkg", "mp3", "mpc", "msv",
        "nmf", "ogg", "oga", "mogg", "opus", "ra", "rm", "raw", "rf64", "sln", "tta", "voc", "vox",
        "wav", "wma", "wv", "webm", "8svx", "cda",
    ];

    // get the audio file from the default audio dir of the OS
    // return an instance of the CommandData and vector of the path if any
    #[tauri::command]
    pub fn fetch_audio() -> Result<CommandData<Vec<File>>, CommandData<()>> {
        // if there is an error getting the audio path, fire an error
        let audio_dir = dirs::audio_dir();
        let Some(audio_dir) = audio_dir else {
            return Err(CommandData::err("error getting the audio dir", ()));
        };

        let entries = search_files("*", &audio_dir)
            .into_iter()
            .filter(|f| ACCEPTABLE_SUFFIXES.contains(&f.file_format.as_str()))
            .collect();

        Ok(CommandData::ok("retrieved all audio files", entries))
    }

    #[cfg(test)]
    mod tests {
        use super::{fetch_audio, ACCEPTABLE_SUFFIXES};
        #[test] // see if there are files in the audio directory path
        fn _fetch_audio_files_() {
            let aud_files = fetch_audio().ok();
            assert!(aud_files.is_some());

            let aud_files = aud_files.unwrap().data.unwrap();
            for file in aud_files {
                let file_format = file.file_format;
                assert!(ACCEPTABLE_SUFFIXES.contains(&file_format.as_str()));
            }
        }
    }
}

pub mod video {
    use super::*;
    static ACCEPTABLE_SUFFIXES: &[&str] = &[
        "mp4", "mkv", "webm", "flv", "vob", "ogv", "ogg", "drc", "gif", "gifv", "mng", "avi",
        "MTS", "MT2S", "TS", "mov", "qt", "wmv", "yuv", "rm", "rmvb", "viv", "asf", "amv", "m4p",
        "m4v", "mpg", "mp2", "mpeg", "mpe", "mpv", "m2v", "svi", "3gp", "3g2", "mxf", "roq", "nsv",
        "f4v", "f4p", "f4a", "f4b",
    ];

    // get the video file from the default video dir of the OS
    // return an instance of the CommandData and vector of the path if any
    #[tauri::command]
    pub fn fetch_videos() -> Result<CommandData<Vec<File>>, CommandData<()>> {
        // if there is an error getting the video path, fire an error
        let video_dir = dirs::video_dir();
        let Some(video_dir) = video_dir else {
            return Err(CommandData::err("error getting the video dir", ()));
        };

        let entries = search_files("*", &video_dir)
            .into_iter()
            .filter(|f| ACCEPTABLE_SUFFIXES.contains(&f.file_format.as_str()))
            .collect();

        Ok(CommandData::ok("retrieved all video files", entries))
    }

    #[cfg(test)]
    mod tests {
        use super::video::{fetch_videos, ACCEPTABLE_SUFFIXES};
        #[test] // see if there are files in the video directory path
        fn _fetch_video_files_() {
            let vid_files = fetch_videos().ok();
            assert!(vid_files.is_some());

            let vid_files = vid_files.unwrap().data.unwrap();
            for file in vid_files {
                let file_format = file.file_format;
                assert!(ACCEPTABLE_SUFFIXES.contains(&file_format.as_str()));
            }
        }
    }
}

#[tauri::command]
pub fn search_home_dir(pattern: &str) -> Result<CommandData<Vec<File>>, CommandData<()>> {
    let home_dir = dirs::home_dir();
    let Some(home_dir) = home_dir else {
        return Err(CommandData::err("error getting the home dir", ()));
    };

    let entries = search_files(pattern, &home_dir);

    Ok(CommandData::ok(
        "searched all files in home directory",
        entries,
    ))
}

// send file from this server to another
// accept path to file as argument
// validate the file existence
// use streams to upload
// the server id is the port on which the peer node run eg -> 23345
#[tauri::command(async)]
pub async fn share_file_with_peer(
    file_path: String,
    server_id: u16,
) -> Result<CommandData<Value>, CommandData<()>> {
    let mut file = tokio::fs::File::open(file_path).await.unwrap();
    let mut vec = Vec::new();
    println!("file content {vec:?}");
    let _ = file.read_to_end(&mut vec).await.unwrap();
    // println!("file content {vec:?}");

    // file.read_to_end(&mut vec).await.unwrap();
    let client = reqwest::Client::new();

    // get the IP address of the share network
    let my_local_ip = ip_manager::autodetect_ip_address()
        .ok()
        .unwrap()
        // .expect("Invalid Ip address detected")
        .parse::<Ipv4Addr>()
        .unwrap();
    let ip_address = format!("http://{:?}:{:?}/upload", my_local_ip, server_id);

    println!("my client id is {ip_address}");
    let _res = client
        .post(&ip_address)
        .header("content-type", "application/octet-stream")
        .body(vec)
        .send()
        .await
        .unwrap();

    println!("the response here {_res:?}");

    // return an instance of the command data
    // Ok(CommandData::new("file successfully sent", true, res))
    Ok(CommandData::ok(
        "file successfully sent",
        json!({
            "success":true,
            // data:r
        }),
    ))
    // todo!()
}
