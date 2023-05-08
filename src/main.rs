use std::{
    env::args,
    fs::{read_dir, DirEntry, metadata}
};

fn main() {
    // take the provided path or default to current dir
    let args = args().skip(1).nth(0);
    let path = if let Some(p) = args { p } else { "./".to_string() };

    let dir_entries = read_dir(&path);
    if let Err(e) = dir_entries.as_ref() {
        println!("Error encountered reading directory '{}': {}", path, e);
        return;
    }
    let dir_entries = dir_entries
        .unwrap()
        .filter_map(|e| {
            if e.is_err() { None }
            else {
                // only directories are needed
                let e = e.unwrap();
                // TODO: symlinks and file errors?
                let meta = metadata(e.path()).unwrap();
                if meta.is_dir() { Some(e) }
                else { None }
            }
        })
        .collect::<Vec<DirEntry>>();
}
