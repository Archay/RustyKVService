use std::net::TcpStream;
use std::io;

use hello_world::LinesCodec;

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8001").unwrap();
    let mut user_input = String::new();
    let stdin = io::stdin();

    let mut codec = LinesCodec::new(stream)?;

    loop {
        println!("Enter a message");
        stdin.read_line(&mut user_input)?;
        user_input.pop();
        println!("{:?}", user_input.as_bytes());
        if user_input == "quit" {
            break;
        }
        println!("Sending to server: {user_input}");
        codec.send_message(&user_input)?;
        let server_res = codec.receive_message()?;
        println!("Server said: {}", server_res);

        user_input.clear();
    }
    Ok(())
}