use std::env;
use std::path::Path;
use std::fs;

struct PythonCompiler {
    version: String,
    path: String,
}



// fn get_python_compiler_path() -> Vec<PythonCompiler> {
//    let path_var = match env::var("PATH") {
//         Ok(path) => path,
//         Err(_) => {
//             println!("PATH variable not found.");
//             return;
//         }
//     };
//     let paths = path_var.split(if cfg!(windows) { ";" } else { ":" });
//     let mut python_compilers : Vec<PythonCompiler> = Vec::new();
//     for path in paths {
//         // Filter out paths that are less likely to be user-installed Python compilers
//         if path.contains("System32") || path.contains("AppData") || path.contains("Common Files") {
//             continue;
//         }
//         let python_path = Path::new(path).join(if cfg!(windows) { "python.exe" } else { "python" });
//         let python3_path = Path::new(path).join(if cfg!(windows) { "python3.exe" } else { "python3" });
//         if fs::metadata(&python_path).is_ok() || fs::metadata(&python3_path).is_ok() {
//             python_compilers
//         }
//     }
// }

fn python_help(){
    println!("help => displays this message ");
    println!("path => displays list of python paths saved ");
}

pub fn command_handler(command : Vec<String>){
    if command.len()  < 3{
        println!("No command provided. Use 'help' with python to get a list of commands.");
        return;
    }
    match command[2].as_str() {
        //"path" =>  get_python_compiler_path(),
        "help" => python_help(),
        _ => println!("No Command found"),
    }
}


