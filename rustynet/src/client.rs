// Runs the TCP client

use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::str;

// Connects to the server and sends any shit from the user.
pub fn run_client() {
    // Trying to connect to the server at addr
    // [*] TODO: Maybe write a fn to let the user choose a port
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            print!("Connected to port 7878");

            loop {
                let mut input = String::new();
                // Read data from stdin
                io::stdin().read_line(&mut input).expect("Failed to read from stdin");
                // Write data gathered to the server. Gotta do it as bytes cus YAY
                stream.write(input.as_bytes()).expect("Failed to write to stream");

                let mut buffer = [0; 512];
                // Read servers response
                match stream.read(&mut buffer) {
                    Ok(_) => {
                        // Print that data
                        println!("Recieved from server: {}", str::from_utf8(&buffer).unwrap());
                    },
                    Err(e) => {
                        // Error Handeling. Please hope to god this never has to be used. I hate
                        // this shit
                        println!("Failed to recieve data from: {}", e);
                        break;
                    }
                }
            }
        },
        // Connect failure. Just connect breh its not that hard.
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    // CLIENT SHIT
}
