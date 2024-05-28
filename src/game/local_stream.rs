use std::{io::{self, Read, Result, Write}, sync::mpsc::{channel, Receiver}, thread};

pub struct LocalStream {
	receiver : Receiver<String>, 
}

impl LocalStream {
	pub fn new() -> Self {
		let (sender, receiver) = channel();
		
		thread::spawn(move || loop {
			let mut buffer = String::new();
			io::stdin().read_line(&mut buffer).unwrap();
			sender.send(buffer).unwrap();
		});
		
		LocalStream {
			receiver,
		}
	}
}

impl Read for LocalStream {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
		let text;
		match self.receiver.try_recv() {
			Ok(input) => text = input,
			Err(_) => text = String::new(),
		}
		
		buf.copy_from_slice(text.as_bytes());
		
		Ok(text.len())
	}
	
	fn read_to_string(&mut self, buf: &mut String) -> Result<usize> {
		let mut text = String::new();
		match self.receiver.try_recv() {
			Ok(input) => text = input,
			Err(_) => {},
		};
		
		buf.clone_from(&text);
		
		Ok(text.len())
	}
}

impl Write for LocalStream {
	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		println!("{:?}", buf);
		Ok(0)
	}
	
	fn flush(&mut self) -> Result<()> {
		Ok(())
	}
}