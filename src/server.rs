use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io;
const MESSAGE_SIZE: usize = 10;
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

fn handle_client(mut socket: TcpStream) -> io::Result<()> {
    println!("New thread created");
    let mut buf = [0u8; MESSAGE_SIZE];
    let mut received: Vec<u8> = vec![];
    let mut run = true;
    while run {
        loop {
            let bytes_read = socket.read(&mut buf)?;
            received.extend_from_slice(&buf[..bytes_read]);
            
            if bytes_read == 0 {
                println!("This thread will be killed");
                run = false;
            }
            if bytes_read < MESSAGE_SIZE {
                break;
            }
        }
        let message = String::from_utf8(received.clone()).unwrap_or("Only valid utf8".to_string());
        
        println!("Received from client: {message}");
        println!("In vector: {:?}", received);
        socket.write(&received)?;
        socket.flush()?;
        received.clear();
        
    }
    Ok(())
}
