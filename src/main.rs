use std::f32::consts::E;
use std::io::prelude::*;
use std::net::TcpStream;

const ECHO_SERVER_ADDRESS: &str = "localhost:1234";

fn main() {
    //connection
    println!("connecting to {}", ECHO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS) {
        //connected
        println!(
            "connected to {}:{}",
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

        //write to socket
        let message: &str = "Hello World!";
        let _ = stream.write(message.as_bytes());
        let _ = stream.flush();
        println!("Sent: {}", message);
    } else {
        println!("Failed to connect to {}", ECHO_SERVER_ADDRESS);
    }
}
