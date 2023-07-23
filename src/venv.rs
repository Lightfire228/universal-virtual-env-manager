use std::path::PathBuf;

use crate::files::{File, Files};

pub mod python;

// Actions are are defined at runtime to support
// allowing them to be defined via config files or other dynamic means
pub struct Venv {
    pub id:      String,
    pub name:    String,
    pub actions: Box<dyn VenvActions>
}

pub trait VenvActions {
    fn check_path(&self, path: &PathBuf) -> bool;
    fn is_active (&self)                 -> bool;
            
    fn activate  (&self, path: &PathBuf) -> ChangeDelta;
    fn deactivate(&self, path: &PathBuf) -> ChangeDelta;
}

impl Venv {
    pub fn check_all(self, files: &Files) -> Files {
        let mut result: Files = Vec::new();

        for f in files {
            if self.actions.check_path(&f) {
                result.push(f.to_path_buf());
            }
        }

        result
    }
}


pub struct ChangeDelta {


}