
use self::super::{Venv, VenvActions};

struct _Python {}

impl Venv {
    pub fn new_python() -> Venv {
    
        Venv {
            id:      String::from("python"),
            name:    String::from("Python"),
            actions: Box::new(_Python {}),
        }
    }
}


impl VenvActions for _Python {
    fn check_path(&self, path: &std::path::PathBuf) -> bool {(
           path.is_dir() 
        && path.ends_with(".venv")
    )}

    fn is_active (&self) -> bool {
        todo!()
    }

    fn activate  (&self, path: &std::path::PathBuf) -> super::ChangeDelta {
        todo!()
    }

    fn deactivate(&self, path: &std::path::PathBuf) -> super::ChangeDelta {
        todo!()
    }
}