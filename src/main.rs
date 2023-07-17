#![allow(unused)]
use std::{fs, io::{self, Result}, env, path::PathBuf};

mod files;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let vars: Vec<(String, String)> = env::vars().collect();

    let cwd = files::get_cwd();

    let x = files::walk_dir(&cwd.unwrap());

    let x = files::expand_tree(&x);

    dbg!(x);

    Ok(())
}
