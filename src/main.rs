use std::env::args;
use walkdir::WalkDir;
use git::test_repo;

pub mod git;

fn main() {
    // take the provided path or default to current dir
    let args = args().skip(1).nth(0);
    let path = if let Some(p) = args { p } else { "./".to_string() };

    let index = WalkDir::new(path);
    for entry in index {
        if let Ok(entry) = entry {
            let path = entry.path().to_string_lossy();
            let meta = entry.metadata();
            if let Err(e) = meta.as_ref() {
                println!("error reading metadata for {}: {}", path, e);
                continue;
            }

            let meta = meta.unwrap();

            // only open the directories that end with .git
            if meta.is_dir() && path.ends_with(".git") {
                if let Err(e) = test_repo(&path) {
                    println!("{}: {}", e, e.root_cause());
                }
            }
        } else {
            println!("Error reading a directory entry: {}", entry.err().unwrap());
        }
    }
}
