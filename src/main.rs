use std::env;
slint::include_modules!();



use crate::commands::python;
mod commands;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn help(){
    println!("Commands:");
    println!("    help: Display this help message");
    println!("    serve: Start a web server");
}
fn main()-> Result<(), slint::PlatformError> {
    let args: Vec<String> = env::args().collect();

    // Check if at least one argument is provided
    if args.len() < 2 {
        println!("No command provided. Use 'help' for usage information.");
        return Ok(());
    }

    match args[1].as_str() {
        "help" => Ok(help()),
        "python" => Ok(commands::python::command_handler(args)),
        "serve" => {
            let ui = Example::new()?;
            ui.run()
        }
        // Add other commands here
        _ => {
            println!("Unknown command {}. Use 'help' for usage information.", args[1]);
            return Ok(());
        }

    }
}


