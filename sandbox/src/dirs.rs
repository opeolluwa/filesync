extern crate dirs;

fn main() {
 
    let home_dir = dirs::home_dir().unwrap();
    println!("Home dir! {home_dir:?}");

    let audio_dir = dirs::audio_dir().unwrap();
    println!("audio dir! {audio_dir:?}");

    let desktop_dir = dirs::desktop_dir().unwrap();
    println!("desktop dir! {desktop_dir:?}");

    let document_dir = dirs::document_dir().unwrap();
    println!("document dir! {document_dir:?}");

    let picture_dir = dirs::picture_dir().unwrap();
    println!("Home dir! {picture_dir:?}");

    let video_dir = dirs::video_dir().unwrap();
    println!("Home dir! {video_dir:?}");
}
