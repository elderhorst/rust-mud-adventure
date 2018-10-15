use std::net::TcpStream;
use std::io::prelude::*;
use std::io::Write;

use commands::commands;
use game::player::Player;
use mud::mudserver::MudServer;

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

    pub fn send(&mut self, text: String) {
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

    fn process_input(&mut self, mut text: String) {
        let params = text.split_off(7);


        commands::handle_command(self, &text, &params);
    }
}