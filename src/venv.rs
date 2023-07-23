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

        files
            .iter()
            .filter(|&x| self.actions.check_path(&x))
            .map   (| x| x.to_path_buf())
            .collect::<Files>()
        
    }
}


pub struct ChangeDelta {


}