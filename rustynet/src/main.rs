// Custom client mod 
mod client;

use std::env;
use std::process;

fn main() {
    // Taking usr input yaaaay
    let argsL Vec<String> = env::args().collect();

    // Check if the usr passed enough shit 
    if args.len() != 2 {
        eprintln!("Usage: rustynet [server|client]");
        process::exit(1);
    }

    // Determine what the usr wants
    match args[1].as_str() {
        "server" => run_server(),
        "client" => client::run_client(),
        _ => {
            eprintln!("Invalid arguments goofy! Use 'server' or 'client' please.");
            process::exit(1);
        }
    }
}

// Start the TCP server
fn run_server() {
    println!("Server starting...");
    // SERVER SHIT GOES HEREEEE 
}
