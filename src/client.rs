use std::net::TcpStream;
use std::io::Write;

pub struct Client {
    id: usize,
    pub stream: TcpStream,
}

impl Client {
    pub fn new(id: usize, stream: TcpStream) -> Client{
        Client {
            id: id,
            stream: stream,
        }
    }

    pub fn write(&mut self, text:String) {
        let response = format!("{}\n", text);
        self.stream.write(response.as_bytes()).expect("Response failed");
    }
}