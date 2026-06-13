use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::atomic::{AtomicU64, Ordering};

static ORDER_SEQUENCE: AtomicU64 = AtomicU64::new(1);

#[derive(Debug)]
enum OrderError {
    InvalidFormat,
    RiskLimitBreached(String),
    NetworkError(String),
}

impl std::fmt::Display for OrderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
        OrderError::InvalidFormat => write!(f, "Invalid order format"),
        OrderError::RiskLimitBreached(msg) => write!(f, "Risk limit breached: {}", msg),
        OrderError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}


#[derive(Debug)]
struct Order {
    id: u64,
    symbol: String,
    price: f64,
    quantity: u32,
    side: String,
    timestamp: u64,
}

fn risk_check(order: &Order) -> Result<(), String> {
    let max_order_value = 100000.0;
    let order_value = order.price * order.quantity as f64;
    
    if order_value > max_order_value {
        return Err(format!(
            "Order value ${} exceeds limit ${}",
            order_value, max_order_value
        ));
    }
    Ok(())
}

fn parse_order(input: &str) -> Result<Order, OrderError> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 4 {
        return Err(OrderError::InvalidFormat);
    }
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
        
    Ok(Order {
        id: ORDER_SEQUENCE.fetch_add(1, Ordering::SeqCst),
        side: parts[0].to_string(),
        symbol: parts[1].to_string(),
        price: parts[2].parse().map_err(|_| OrderError::InvalidFormat)?,
        quantity: parts[3].parse().map_err(|_| OrderError::InvalidFormat)?,
        timestamp,
    })
}

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
                if n == 0 { return; }
            
                let input = String::from_utf8_lossy(&buf[..n]);
                match parse_order(&input) {
                    Ok(order) => {
                        println!("Parsed order: {:?}", order);
                        match risk_check(&order) {
                            Ok(()) => {
                                socket.write_all(
                                    format!(
                                        "ACK: {} {} x{} @ ${}\n",
                                        order.side, order.symbol,
                                        order.quantity, order.price
                                    )
                                    .as_bytes()
                                )
                                .await.unwrap();
                            }
                            Err(reason) => {
                                socket.write_all(
                                    format!("REJECTED: {}\n", reason).as_bytes()
                                )
                                .await.unwrap();
                            }
                        }
                    }
                    Err(_) => {
                        socket.write_all(b"ERROR: invalid order format\n")
                            .await.unwrap();
                    }
                }
            }
        });
    }
}