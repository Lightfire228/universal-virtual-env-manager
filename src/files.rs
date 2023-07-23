use std::path::PathBuf;
use std::path::Path;
use std::io::Result;
use std::fs;
use std::env;

pub type File  = PathBuf;
pub type Files = Vec<File>;

pub fn walk_path(start: &File) -> Files {

    start
        .ancestors()
        .map(|x| x.components().collect())
        .collect()
    
}

pub fn expand_tree(paths: &Files) -> Result<Files> {

    let mut results: Files = Vec::new();

    for path in paths {
        for file in path.read_dir()? {
            let file = file?;

            results.push(file.path());
        }
    }

    Ok(results)
}


pub fn get_cwd() -> Result<File> {
    env::current_dir()
}
