use glob::glob;
extern crate glob;
use walkdir::WalkDir;

fn main() {
     let audio_dir = dirs::video_dir().unwrap();
    println!("audio dir! {audio_dir:?}\n\n");
    for entry in WalkDir::new(audio_dir) {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    } 
    println!("hey fool");
}
