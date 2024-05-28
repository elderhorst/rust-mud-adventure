use std::{io::{Read, Result, Write}, net::TcpStream};

use super::local_stream::LocalStream;

pub enum GameStream {
	Local(LocalStream),
	Remote(TcpStream),
}

impl Read for GameStream {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
		match self {
			GameStream::Local(stream) => stream.read(buf),
			GameStream::Remote(stream) => stream.read(buf),
		}
	}
	
	fn read_to_string(&mut self, buf: &mut String) -> Result<usize> {
		match self {
			GameStream::Local(stream) => stream.read_to_string(buf),
			GameStream::Remote(stream) => stream.read_to_string(buf),
		}
	}
}

impl Write for GameStream {
	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		match self {
			GameStream::Local(stream) => stream.write(buf),
			GameStream::Remote(stream) => stream.write(buf),
		}
	}
	
	fn flush(&mut self) -> Result<()> {
		match self {
			GameStream::Local(stream) => stream.flush(),
			GameStream::Remote(stream) => stream.flush(),
		}
	}
}
