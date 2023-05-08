use std::env::args;
use walkdir::WalkDir;

fn main() {
    // take the provided path or default to current dir
    let args = args().skip(1).nth(0);
    let path = if let Some(p) = args { p } else { "./".to_string() };

    let index = WalkDir::new(path);
}
