use std::io::{BufRead, Write, self};
use std::net::TcpStream;

pub struct LinesCodec {
    reader: io::BufReader<TcpStream>,
    writer: io::LineWriter<TcpStream>,
}

impl LinesCodec {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        let writer = io::LineWriter::new(stream.try_clone()?);
        let reader = io::BufReader::new(stream);
        Ok(Self{reader, writer})
    }

    pub fn send_message(&mut self, message: &str) -> io::Result<()> {
        self.writer.write(&message.as_bytes())?;
        self.writer.write(&['\n' as u8])?;
        Ok(())
    }

    pub fn receive_message(&mut self) -> io::Result<String> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        line.pop();
        Ok(line)
    }
}