use std::{fs, io, path::PathBuf};
extern crate dirs;

fn read_dir() -> io::Result<()> {
    let home_dir = dirs::home_dir().unwrap();
    let mut entries = fs::read_dir(home_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();
    println!("{:#?}", entries);

    Ok(())
}
