use std::net::TcpStream;
use std::io::prelude::*;
use std::io::Write;

use player::Player;

pub struct Client {
    id: usize,
    pub stream: TcpStream,
    player: Player,
}

impl Client {
    pub fn new(id: usize, stream: TcpStream) -> Client{
        Client {
            id: id,
            stream: stream,
            player: Player::new(),
        }
    }

    pub fn write(&mut self, text:String) {
        let response = format!("{}\n", text);
        self.stream.write(response.as_bytes()).expect("Response failed");
    }

    pub fn update(&mut self) {
        let mut buffer = String::new();
        let result = self.stream.read_to_string(&mut buffer);
        buffer = buffer.trim().to_string();

        if buffer.len() != 0 {
            self.process_input(buffer);
        }

        // There is probably an error this should handle.
        match result {
            Ok(_) => {},
            _ => {},
        }
    }

    fn process_input(&mut self, text:String) {
        if text == "hello" {
            println!("!!!!!!!!!!!!!!!");
        }

        self.write(format!("text recived: {}", text));
    }
}