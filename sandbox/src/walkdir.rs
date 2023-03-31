use glob::glob;
extern crate glob;
use walkdir::WalkDir;

fn main() {
     let audio_dir = dirs::audio_dir().unwrap();
    println!("audio dir! {audio_dir:?}");
    for entry in WalkDir::new(audio_dir) {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    } 
    println!("hey fool");
}
