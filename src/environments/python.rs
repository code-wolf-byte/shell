

fn getPythonCompilerPath() {
    let path_var = match env::var("PATH") {
        Ok(path) => path,
        Err(_) => {
            println!("PATH variable not found.");
            return;
        }
    };

    let paths = path_var.split(if cfg!(windows) { ";" } else { ":" });

    for path in paths {
        // Filter out paths that are less likely to be user-installed Python compilers
        if path.contains("System32") || path.contains("AppData") || path.contains("Common Files") {
            continue;
        }

        let python_path = Path::new(path).join(if cfg!(windows) { "python.exe" } else { "python" });
        let python3_path = Path::new(path).join(if cfg!(windows) { "python3.exe" } else { "python3" });

        
        
    }

}