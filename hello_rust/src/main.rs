use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Order server listening on port 8080");
    
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("New connection from: {}", addr);
        
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                let n = socket.read(&mut buf).await.unwrap();
                if n==0 {
                    println!("Connection closed");
                    return;
                }
                
                let order = String::from_utf8_lossy(&buf[..n]);
                println!("Received order: {}", order);
                socket.write_all(b"Order received\n").await.unwrap();
            }
        });
    }
}