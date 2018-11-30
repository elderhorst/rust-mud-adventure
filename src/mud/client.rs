use std::net::TcpStream;
use std::io::prelude::*;
use std::io::Write;

use game::player::Player;
use mud::updatedata::UpdateData;

pub struct Client {
    id: usize,
    pub stream: TcpStream,
    pub player: Player,
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
        let response = format!("{}\n", text.trim());
        self.stream.write(response.as_bytes()).expect("Response failed");
    }

    pub fn update(&mut self) -> UpdateData {
        let mut buffer = String::new();
        let result = self.stream.read_to_string(&mut buffer);
        buffer = buffer.trim().to_string();

        if buffer.len() != 0 {
            return self.process_input(&buffer);
        }
        else {
            return UpdateData {
                id: self.id,
                command: "".to_string(),
                params: "".to_string(),
            };
        }

        // There is probably an error this should handle.
        /*match result {
            Ok(_) => {},
            _ => {},
        }*/
    }

    fn process_input(&mut self, mut text: &String) -> UpdateData {
        let command_length = self.get_command_length(text);
        let command = &text[0..command_length];
        let mut params = "";

        if command_length < text.len() - 1 {
            params = &text[command_length + 1..text.len()];
        }

        let data = UpdateData {
            id: self.id.clone(),
            command: command.to_string(),
            params: params.to_string(),
        };

        return data;
    }

    fn get_command_length(&self, s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        return s.len();
    }
}