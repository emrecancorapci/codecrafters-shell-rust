use std::{
    env,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

pub fn get_exec_path_string(exec: &str) -> Result<String, Error> {
    for path in env::var("PATH").unwrap().split(":") {
        let path = format!("{}/{}", path, exec);

        if std::fs::metadata(&path).is_ok() {
            return Ok(path);
        }
    }

    Err(Error::new(ErrorKind::NotFound, format!("{}: command not found", exec)))
}

pub fn get_exec_path(exec: &str) -> Result<PathBuf, Error> {
    let binding = env::var("PATH").unwrap();
    let paths: Vec<&str> = binding.split(":").collect();
    let cmd_path = Path::new(&paths[0]).join(exec);

    if cmd_path.exists() {
        return Ok(cmd_path);
    } else {
        Err(Error::new(ErrorKind::NotFound, format!("{}: command not found", exec)))
    }
}
