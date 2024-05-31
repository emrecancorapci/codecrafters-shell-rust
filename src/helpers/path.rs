use std::{ path::{ Path, PathBuf }, env };

pub fn get_exec_path_string(exec: &str) -> Result<String, &str> {
    for path in env::var("PATH").unwrap().split(":") {
        let path = format!("{}/{}", path, exec);

        if std::fs::metadata(&path).is_ok() {
            return Ok(path);
        }
    }

    Err("exec not found")
}

pub fn get_exec_path(exec: &str) -> Result<PathBuf, &str> {
    let binding = env::var("PATH").unwrap();
    let paths: Vec<&str> = binding.split(":").collect();
    let cmd_path = Path::new(&paths[0]).join(exec);

    if cmd_path.exists() {
        return Ok(cmd_path);
    } else {
        Err("exec not found")
    }
}
