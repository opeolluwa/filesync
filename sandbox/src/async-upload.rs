// see guide here 
//https://www.reddit.com/r/rust/comments/lamcym/best_way_to_upload_large_files_using_http_in_rust/
use tokio::io::{self, AsyncReadExt};
use tokio::fs::File; 

 let mut file = File::open("test_data/bigvideo.mp4").await.unwrap();
 let mut vec = Vec::new(); file.read_to_end(&mut vec);
 let client = reqwest::Client::new(); let res = client.post("http://localhost:4000/upload/test").header("content-type","application/octet-stream") 
.body(vec) 
.send().await.unwrap();
