use std::{
    env,
    path::{Path, PathBuf},
};

pub trait ExecutionPath {
    fn get_exec_path(&self) -> Option<PathBuf>;
}

impl ExecutionPath for &String {
    fn get_exec_path(&self) -> Option<PathBuf> {
        for path in env::var("PATH").unwrap().split(":") {
            let cmd_path = Path::new(path).join(self);

            if cmd_path.exists() {
                return Some(cmd_path);
            }
        }

        None
    }
}
