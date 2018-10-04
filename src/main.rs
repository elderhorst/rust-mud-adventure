mod client;
mod mudserver;

use mudserver::MudServer;

use std::net::{TcpStream, TcpListener};
use std::io::ErrorKind;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    listener.set_nonblocking(true).expect("Error: cannot set non-blocking");

    let mut mudserver = MudServer::new();

    for mut stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                mudserver.add_client(stream);
                mudserver.write_to_client(0, format!("hello world {}", 0));
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}