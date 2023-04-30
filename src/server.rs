use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io;

use hello_world::LinesCodec;
fn main() {
    println!("Listening on port 8001");
    let listener = TcpListener::bind("127.0.0.1:8001").unwrap();
    loop {
        match listener.accept() {
            Ok((socket, addr)) => {
                println!("New client:{addr:?}");
                thread::spawn(|| {handle_client(socket)});
            },
            Err(e) => println!("Error {e}"),
        }
    }
}

fn handle_client(stream: TcpStream) -> io::Result<()> {
    println!("New thread created");
    let mut codec = LinesCodec::new(stream)?;
    loop {
        
        let message = codec.receive_message()?;
        if message.is_empty() {
            println!("Client disconnected");
            break;
        }
        
        println!("Received from client: {message}");
        println!("In vector: {:?}", message.as_bytes());
        codec.send_message(&message)?;
    }
    Ok(())
}
