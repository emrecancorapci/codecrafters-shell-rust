pub fn get_exec_path(exec: &str) -> Result<String, &str> {
    for path in std::env::var("PATH").unwrap().split(":") {
        let path = format!("{}/{}", path, exec);

        if std::fs::metadata(&path).is_ok() {
            return Ok(path);
        }
    }

    Err("exec not found")
}