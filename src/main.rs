use std::f32::consts::E;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
const ECHO_SERVER_ADDRESS: &str = "localhost:1234";
#[tokio::main]
async fn main() {
    //connection
    println!("connecting to {}", ECHO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS).await {
        //connected
        println!(
            "connected to {}:{}",
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

        //write to socket
        let message: &str = "Hello World!";
        let _ = stream.write_all(message.as_bytes()).await;
        println!("Sent: {}", message);

        // receive the result as client.
        let mut buffer = [0; 2048];
        let len = stream.read(&mut buffer).await.unwrap();
        let message = String::from_utf8_lossy(&buffer);
        println!("received {}", message);
    } else {
        println!("Failed to connect to {}", ECHO_SERVER_ADDRESS);
    }
}
