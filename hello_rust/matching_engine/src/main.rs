/*
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
*/

// 2nd code:

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
struct Order {
    symbol: String,
    price: f64,
    quantity: u32,
    side: String,
}

fn parse_order(input: &str) -> Option<Order> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len()  !=4 {
        return None;
    }
    Some(Order {
        side: parts[0].to_string(),
        symbol: parts[1].to_string(),
        price: parts[2].parse().ok()?,
        quantity: parts[3].parse().ok()?,
    })
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Order server listening on port 8080");
    
    loop {
        let (mut socket,addr) = listener.accept().await.unwrap();
        println!("New connection from: {}", addr);
    
    
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                let n = socket.read(&mut buf).await.unwrap();
                if n == 0 {return; }
            
                let input = String::from_utf8_lossy(&buf[..n]);
                match parse_order(&input) {
                    Some(order) => {
                        println!("Parsed order: {:?}", order);
                        socket.write_all(
                            format!("ACK: {} {} x{} @ ${}\n",
                                order.side, order.symbol,
                                order.quantity, order.price)
                            .as_bytes())
                        .await.unwrap();
                    }
                    None => {
                        socket.write_all(b"ERROR: invalid order format\n").await.unwrap();
                    }
                }
            }
        });
    }
}