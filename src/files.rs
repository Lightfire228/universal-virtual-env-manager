use std::path::PathBuf;
use std::path::Path;
use std::io::Result;
use std::fs;
use std::env;

mod node;

pub fn walk_dir(start: &PathBuf) -> Vec<PathBuf> {

    start
        .ancestors()
        .map(|x| x.components().collect())
        .collect()
    
}

pub fn expand_tree(paths: &Vec<PathBuf>) -> Result<Box<Vec<PathBuf>>> {

    let mut results: Box<Vec<PathBuf>> = Box::new(Vec::new());

    for path in paths {
        for file in path.read_dir()? {
            let file = file?;

            results.push(file.path());
        }
    }

    Ok(results)
}


pub fn get_cwd() -> Result<PathBuf> {
    let e = env::current_dir()?;

    println!("CWD: {}", e.to_str().unwrap());

    Ok(e)
}
