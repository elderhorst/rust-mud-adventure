use crate::game::game_stream::GameStream;

pub struct Client {
    pub stream: GameStream,
}

impl Client {
    pub fn new(stream: GameStream) -> Client {
        Client {
            stream: stream,
        }
    }
}