use std::env;
use crate::utils::help;
// use std::fs;
// use std::net::{TcpStream, TcpListener};
// use webbrowser;
mod utils;
fn main(){
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    utils::commands::help();
    // if query.eq("help"){
    //     help::help();
    // } else if query.eq("serve"){
    //     let contents = fs::read_to_string("index.html").expect("Something went wrong reading the file");
    //     println!("Serving index.html");
    //     println!("Press Ctrl+C to stop");
    //     let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    //     println!("Listening for connections on port {}", 8080);
    //     if webbrowser::open("http://localhost:8080").is_ok() {
    //         println!("Opened browser");
    //     }
    //     else {
    //         panic!("Failed to open browser");
    //     }
    // }
}



