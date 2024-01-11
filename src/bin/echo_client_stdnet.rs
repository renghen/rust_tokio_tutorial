use std::io::prelude::*;
use std::net::TcpStream;

const ECHO_SERVER_ADDRESS: &str = "127.0.0.1:1234";

fn main() {
    println!("connecting to {}", ECHO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS) {
        let address = stream.local_addr().unwrap();
        println!(
            "connected to echo server {}:{}",
            address.ip(),
            address.port()
        );

        // writing to socket
        let message = "Hello World!";
        let _ = stream.write(message.as_bytes());
        let _ = stream.flush();
        println!("sending  {}", message);

        let mut buffer = [0u8; 1024];

        //reading from socket
        let length = stream.read(&mut buffer).unwrap();
        let recieved = String::from_utf8(buffer[0..length].to_vec()).unwrap();
        println!("recieved {}", recieved);
    } else {
        println!("Error connecting to echo server {}", ECHO_SERVER_ADDRESS);
    }
}
