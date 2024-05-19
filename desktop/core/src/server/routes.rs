use axum::response::Html;
use axum::{BoxError, Json};
// get documents, audio, video, e.t.c and render the in the browser
use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use futures::{Stream, TryStreamExt};
use hyper::header;
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::{self, BufWriter};
use tokio_util::io::{ReaderStream, StreamReader};

use axum::body::{Bytes, StreamBody};
use axum::extract::Multipart;
use serde_json::json;
use serde_json::Value;
use std::fs;

use crate::utils::{system_info::SystemInformation, CommandData};
use crate::UPLOAD_DIRECTORY;
#[derive(Debug, Serialize, Deserialize)]

/// destructure query parameter
pub struct QueryParams {
    pub file_path: String,
}
/// accept file path amd return the file
pub async fn download_file(Query(params): Query<QueryParams>) -> impl IntoResponse {
    let QueryParams { file_path } = params;

    let Some(file) = tokio::fs::File::open(file_path).await.ok() else {
        return Err((
            StatusCode::NOT_FOUND,
            axum::response::Json(serde_json::json!({
            "success":false,
            "message":String::from("The requested file could not be found!"),
            })),
        ));
    };
    // TODO use mime guess
    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    let body = StreamBody::new(stream);

    let headers = [
        (header::CONTENT_TYPE, "text/toml; charset=utf-8"),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"Cargo.toml\"",
        ),
    ];

    Ok((headers, body))
}

/// return the system information
pub async fn system_information() -> (StatusCode, Json<CommandData<SystemInformation>>) {
    (
        StatusCode::OK,
        axum::Json(CommandData::ok(
            "connected system information ",
            SystemInformation::new(),
        )),
    )
}

// return an html page to receive file upload
#[allow(unused)]
pub async fn file_upload_form_new() -> Html<&'static str> {
    Html(
        r#"
<!DOCTYPE html>
<html lang='en'>
  <head>
    <meta charset='UTF-8' />
    <meta name='viewport' content='width=device-width, initial-scale=1.0' />
    <title>Document</title>
    <style>
      /* Import Google font — Poppins */
      @import url('https://fonts.googleapis.com/css2?family=Poppins:wght@400;500;600&display=swap');
      * {
        margin: 0;
        padding: 0;
        box-sizing: border-box;
        font-family: 'Poppins', sans-serif;
      }
      body {
        display: flex;
        align-items: center;
        justify-content: center;
        min-height: 100vh;
        background: #f2f2f2;
        padding: 10px 20px;
      }

      ::selection {
        color: #fff;
        background: #6990f2;
      }
      .wrapper {
        width: 430px;
        background: #fff;
        border-radius: 5px;
        padding: 30px;
        box-shadow: 7px 7px 12px rgba(0, 0, 0, 0.05);
      }
      .wrapper header {
        color: #6990f2;
        font-size: 27px;
        font-weight: 600;
        text-align: center;
      }
      .wrapper form {
        height: 167px;
        display: flex;
        cursor: pointer;
        margin: 30px 0;
        align-items: center;
        justify-content: center;
        flex-direction: column;
        border-radius: 5px;
        border: 2px dashed #6990f2;
      }
      form :where(i, p) {
        color: #6990f2;
      }
      form i {
        font-size: 50px;
      }
      form p {
        margin-top: 15px;
        font-size: 16px;
      }

      section .row {
        margin-bottom: 10px;
        background: #e9f0ff;
        list-style: none;
        padding: 15px 20px;
        border-radius: 5px;
        display: flex;
        align-items: center;
        justify-content: space-between;
      }
      section .row i {
        color: #6990f2;
        font-size: 30px;
      }
      section .details span {
        font-size: 14px;
      }
      .progress-area .row .content {
        width: 100%;
        margin-left: 15px;
      }
      .progress-area .details {
        display: flex;
        align-items: center;
        margin-bottom: 7px;
        justify-content: space-between;
      }
      .progress-area .content .progress-bar {
        height: 6px;
        width: 100%;
        margin-bottom: 4px;
        background: #fff;
        border-radius: 30px;
      }
      .content .progress-bar .progress {
        height: 100%;
        width: 0%;
        background: #6990f2;
        border-radius: inherit;
      }
      .uploaded-area {
        max-height: 232px;
        overflow-y: scroll;
      }
      .uploaded-area.onprogress {
        max-height: 150px;
      }
      .uploaded-area::-webkit-scrollbar {
        width: 0px;
      }
      .uploaded-area .row .content {
        display: flex;
        align-items: center;
      }
      .uploaded-area .row .details {
        display: flex;
        margin-left: 15px;
        flex-direction: column;
      }
      .uploaded-area .row .details .size {
        color: #404040;
        font-size: 11px;
      }
      .uploaded-area i.fa-check {
        font-size: 16px;
      }
    </style>
  </head>
  <body>
    <div class='wrapper'>
      <header>File Uploader</header>
      <form action='/upload'>
        <input class='file-input' type='file' name='file' hidden />
        <i class='fas fa-cloud-upload-alt'></i>
        <p>Browse File to Upload</p>
      </form>
      <section class='progress-area'></section>
      <section class='uploaded-area'></section>
    </div>
    <script>
      const form = document.querySelector('form'),
        fileInput = document.querySelector('.file-input'),
        progressArea = document.querySelector('.progress-area'),
        uploadedArea = document.querySelector('.uploaded-area');

      form.addEventListener('click', () => {
        fileInput.click();
      });

      fileInput.onchange = ({ target }) => {
        let file = target.files[0];
        if (file) {
          let fileName = file.name;
          if (fileName.length >= 12) {
            let splitName = fileName.split('.');
            fileName = splitName[0].substring(0, 13) + '… .' + splitName[1];
          }
          uploadFile(fileName);
        }
      };

      function uploadFile(name) {
        let xhr = new XMLHttpRequest();
        xhr.open('POST', '/');
        xhr.upload.addEventListener('progress', ({ loaded, total }) => {
          let fileLoaded = Math.floor((loaded / total) * 100);
          let fileTotal = Math.floor(total / 1000);
          let fileSize;
          fileTotal < 1024
            ? (fileSize = fileTotal + ' KB')
            : (fileSize = (loaded / (1024 * 1024)).toFixed(2) + ' MB');
          let progressHTML = `<li class='row'>
      <i class='fas fa-file-alt'></i>
      <div class='content'>
      <div class='details'>
      <span class='name'>${name} • Uploading</span>
      <span class='percent'>${fileLoaded}%</span>
      </div>
      <div class='progress-bar'>
      <div class='progress' style='width: ${fileLoaded}%'></div>
      </div>
      </div>
      </li>`;
          uploadedArea.classList.add('onprogress');
          progressArea.innerHTML = progressHTML;
          if (loaded == total) {
            progressArea.innerHTML = '';
            let uploadedHTML = `<li class='row'>
      <div class='content upload'>
      <i class='fas fa-file-alt'></i>
      <div class='details'>
      <span class='name'>${name} • Uploaded</span>
      <span class='size'>${fileSize}</span>
      </div>
      </div>
      <i class='fas fa-check'></i>
      </li>`;
            uploadedArea.classList.remove('onprogress');
            uploadedArea.insertAdjacentHTML('afterbegin', uploadedHTML);
          }
        });
        let data = new FormData(form);
        xhr.send(data);
      }
    </script>
  </body>
</html>

  
   "#,
    )
}

/// return an html page to receive file upload
pub async fn file_upload_form() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>File Upload Form</title>
    <style>
      body {
        font-family: "Arial", sans-serif;
        background-color: #f5f5f5;
        margin: 0;
        padding: 0;
        display: flex;
        align-items: center;
        justify-content: center;
        height: 100vh;
      }

      .container {
        background-color: #ffffff;
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        overflow: hidden;
        width: 400px;
      }

      .form-container {
        padding: 20px;
      }

      label {
        display: block;
        margin-bottom: 8px;
        color: #333;
      }

      input[type="file"] {
        width: 100%;
        padding: 10px;
        margin-bottom: 16px;
        box-sizing: border-box;
        border: 1px solid #ccc;
        border-radius: 4px;
      }

      input[type="submit"] {
        background-color: #4caf50;
        color: #fff;
        padding: 10px 15px;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-size: 16px;
      }

      input[type="submit"]:hover {
        background-color: #45a049;
      }
    </style>
  </head>

  <body>
    <div class="container">
      <div class="form-container">
        <h2>File Upload Form</h2>
        <form action="/upload" method="post" enctype="multipart/form-data">
          <label for="file">Choose a file:</label>
          <input type="file" id="file" name="file" multiple required />
          <input type="submit" value="Upload" />
        </form>
      </div>
    </div>
  </body>
</html>

   "#,
    )
}

// Handler that accepts a multipart form upload and streams each field to a file.
pub async fn accept_file_upload(
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<Value>), (StatusCode, String)> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = if let Some(file_name) = field.file_name() {
            file_name.to_owned()
        } else {
            continue;
        };

        stream_to_file(&file_name, field).await?;
    }

    Ok((
        StatusCode::OK,
        Json(json!({
            "Success":true,
            "message":"file saved"
        })),
    ))
}

// Save a `Stream` to a file
async fn stream_to_file<S, E>(path: &str, stream: S) -> Result<(), (StatusCode, String)>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    if !path_is_valid(path) {
        return Err((StatusCode::BAD_REQUEST, "Invalid path".to_owned()));
    }

    async {
        // Convert the stream into an `AsyncRead`.
        let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        //create wi-share directory in the downloads path dir and / save files to $DOWNLOADS/wi-share
        let os_default_downloads_dir = dirs::download_dir().unwrap();
        let upload_path = format!(
            "{downloads_dir}/{upload_dir}",
            downloads_dir = os_default_downloads_dir.display(),
            upload_dir = UPLOAD_DIRECTORY.as_str()
        );
        // create the uploads path if not exist
        let _ = fs::create_dir_all(&upload_path);

        // Create the file. `File` implements `AsyncWrite`.
        let path = std::path::Path::new(&upload_path).join(path);
        let mut file = BufWriter::new(File::create(path).await?);

        // Copy the body into the file.
        tokio::io::copy(&mut body_reader, &mut file).await?;

        Ok::<_, io::Error>(())
    }
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

// to prevent directory traversal attacks we ensure the path consists of exactly one normal
// component
fn path_is_valid(path: &str) -> bool {
    let path = std::path::Path::new(path);
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            return false;
        }
    }

    components.count() == 1
}

// 404 handler
pub async fn handle_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        axum::response::Json(serde_json::json!({
        "success":false,
        "message":String::from("The requested resource does not exist on this server!"),
        })),
    )
}

/// health check handler
pub async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        axum::response::Json(serde_json::json!({
        "success":true,
        "message":String::from("Server is ready to accept connection"),
        })),
    )
}

/// ping the server
pub async fn ping_server() -> impl IntoResponse {
    "FileSync Server 1.0.0"
}

/// for a given file path, return the file the the used as a downloadable one
pub async fn get_file(Query(QueryParams { file_path }): Query<QueryParams>) -> impl IntoResponse {
    // `File` implements `AsyncRead`
    let file = match tokio::fs::File::open(file_path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    let body = StreamBody::new(stream);

    let headers = [
        (header::CONTENT_TYPE, "text/toml; charset=utf-8"),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"Cargo.toml\"",
        ),
    ];

    Ok((headers, body))
}

#[cfg(test)]
mod basic_endpoints {
    use crate::server::router;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    // use serde_json::{json, Value};
    use tower::ServiceExt;
    // test the server base url
    // for example ->  http://loccalhost:4835
    // the index route should return hello world
    #[tokio::test]
    async fn base_url() {
        let app = router::app();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // response status code should be 200
        assert_eq!(response.status(), StatusCode::OK);
    }

    // 404 path
    #[tokio::test]
    async fn not_found_handler() {
        let app = router::app();

        // the 404 handle should return this json
        // it will return a NOT_FOUND  status code
        // the test will test for the validity of  this.
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/not-found-error")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // assert  the the status code is 404
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        // println!(" the not-found-endpoint response is {response:?}");
    }
}
