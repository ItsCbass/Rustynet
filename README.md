# Rustynet - A TCP Server/Client written in Rust 🚀
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

This is just a simple TCP server and client application written in Rust. It's nothing special, just a small project to get into Rust and network programming.

## Features 🌟

- **TCP Server**: Listens for incoming connections.
- **TCP Client**: Connects to the server and sends data.
- **Multithreading**: Handles multiple client connections simultaneously.
- **Echo Functionality**: The server echoes back any data it receives.

## Prerequisites ✅

All you need is Rust installed on your system. If not, you can install it from [the official website](https://www.rust-lang.org/learn/get-started).

## Installation 🛠

Clone the repository to your local machine:

```bash
git clone https://github.com/your-username/rust-tcp-client-server.git
cd rust-tcp-client-server
```

## Usage 💡
Compile the project first:
```bash
cargo build
```

Run the server:
```bash
cargo run -- server
```
This will start a new server on '127.0.0.1:7878'

Run the client:
```bash
cargo run -- client
```
After starting the client, type a message into your terminal and the server will tell you if it's been recieved or gone. If it's successful, your message will be printed back to you.
