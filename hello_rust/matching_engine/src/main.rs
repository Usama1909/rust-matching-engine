use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::time::Duration;

#[derive(Debug, Clone)]

struct Order {
    id: u32,
    symbol: String,
    price: f64,
    quantity: u32,
    side: String,
}
impl Order {
    fn new(id: u32, symbol: &str, price: f64, quantity: u32, side: &str) -> Order {
        Order {
            id,
            symbol:String::from(symbol),
            price,
            quantity,
            side: String::from(side),
        }
    }
}

struct OrderBook{
    bids: Vec<Order>,
    asks: Vec<Order>,
}

impl OrderBook{
    fn new() -> OrderBook {
        OrderBook{
            bids: Vec::new(),
            asks: Vec::new(),
        }
    }
    
    fn add_order(&mut self, order: Order) {
        if order.side == "BUY" {
            self.bids.push(order);
        } else {
            self.asks.push(order);
        }
    }
    
    fn match_orders(&mut self) {
        self.bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
        self.asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
        
        while !self.bids.is_empty() && !self.asks.is_empty() {
            let bid = &self.bids[0];
            let ask = &self.asks[0];
            
            if bid.price >= ask.price{
                let qty = bid.quantity.min(ask.quantity);
                println!("MATCH: {} units of {} at ${}", qty, bid.symbol, ask.price);
                self.bids.remove(0);
                self.asks.remove(0);
            } else {
                break;
            }
        }
        
    }
    fn print_book(&self){
        println!("BIDS: {:?}", self.bids.len());
        println!("ASLS: {:?}", self.asks.len());
    }
}
    fn main() {
    let book = Arc::new(Mutex::new(OrderBook::new()));
    let (sender, receiver) = mpsc::channel::<Order>();
    
    let book_clone = Arc::clone(&book);
    let engine = thread::spawn(move || {
        for order in receiver {
            println!("Engine received: {:?}", order);
            let mut b = book_clone.lock().unwrap();
            b.add_order(order);
            b.match_orders();
        }
    });
    let orders = vec![
    Order::new(1, "NVDA", 500.0, 10, "BUY"),
    Order::new(2, "NVDA", 495.0, 5, "SELL"),
    Order::new(3, "AAPL", 150.0, 20, "BUY"),
    Order::new(4, "AAPL", 148.0, 20, "SELL"),
    ];
    for order in orders{
        sender.send(order).unwrap();
        thread::sleep(Duration::from_millis(10))
    }
    
    drop(sender);
    engine.join().unwrap();
    
    let b = book.lock().unwrap();
    b.print_book();
}
