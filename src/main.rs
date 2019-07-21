extern crate rust_mud;

use rust_mud::mud::mudserver::MudServer;

use std::net::TcpListener;
use std::io::ErrorKind;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    listener.set_nonblocking(true).expect("Error: cannot set non-blocking");

    let mut mudserver = MudServer::new();

    println!("Started server on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                mudserver.add_client(stream);
                mudserver.send_message(0, format!("hello world {}", 0));

                println!("Connection opened");
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }

        mudserver.update();
    }
}