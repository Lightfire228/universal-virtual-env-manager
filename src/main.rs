#![allow(unused)]

mod file_tree;
mod files;
mod venv;

use std::{fs, io::{self, Result}, env, path::PathBuf};


use venv::{Venv, python};


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let vars: Vec<(String, String)> = env::vars().collect();

    let cwd = files::get_cwd();

    let x = files::walk_path(&cwd.unwrap());

    let x = files::expand_tree(&x).unwrap();

    let venv = Venv::new_python();

    let found = venv.check_all(&x);

    dbg!(found);

    // dbg!(x);

    Ok(())
}


