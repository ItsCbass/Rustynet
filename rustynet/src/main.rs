// Custom client mod 
mod client;

use std::env;
use std::process;

fn main() {
    // Taking usr input yaaaay
    let args: Vec<String> = env::args().collect();

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

// Std networking libs
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

// Start the TCP server
fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Couldnt bind :(");
    println!("Server listening on port 7878 YAY");
    
    // Setup a loop for incomming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawns a new thread for any new connection. I love rusts sys threads bro.
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => { /* Connection faileddddd */ }
        }
    }
}

// Client connections 
// @args - 'stream'
// TCP stream between the server and the client. God help us if this doesnt work
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    // Read data from the stream in a loop
    while match stream.read(&mut buffer) {
        Ok(size) => {
            // We're gonna print out anything the servers reciving
            stream.write(&buffer[0..size]).unwrap();
            true
        },
        Err(_) => { // Error handling when something ineviably fucks up <3
            println!("Something fucked up, terminating the connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false
        }
    } {}
}
