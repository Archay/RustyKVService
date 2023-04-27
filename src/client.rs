use std::io::Write;
use std::io::Read;
use std::net::TcpStream;
use std::io;
const MESSAGE_SIZE: usize = 10;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8001").unwrap();
    let mut user_input = String::new();
    let stdin = io::stdin();

    let mut buf = [0u8; MESSAGE_SIZE];
    let mut received: Vec<u8> = vec![];
    loop {
        println!("Enter a message");
        stdin.read_line(&mut user_input)?;
        if user_input == "quit\n" {
            break;
        }
        println!("Sending to server: {user_input}");
        stream.write_all(&user_input.as_bytes())?;
        stream.flush()?;
        loop {
            let bytes_read = stream.read(&mut buf)?;
            received.extend_from_slice(&buf[..bytes_read]);
            
            if bytes_read < MESSAGE_SIZE {
                break;
            }
        }
        let server_res = String::from_utf8(received.clone()).unwrap_or("Only valid utf8".to_string());
        received.clear();
        println!("Server said: {}", server_res);

        user_input.clear();
    }
    Ok(())
}