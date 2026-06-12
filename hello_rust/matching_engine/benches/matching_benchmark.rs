use criterion::{criterion_group, criterion_main, Criterion};

#[derive(Debug, Clone)]
struct Order {
    symbol: String,
    price: f64,
    quantity: u32,
    side: String,
}

struct OrderBook {
    bids: Vec<Order>,
    asks: Vec<Order>,
}

impl OrderBook {
    fn new() -> OrderBook {
        OrderBook {
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
        
        while !self.bids.is_empty()&& !self.asks.is_empty() {
            if self.bids[0].price >= self.asks[0].price {
                self.bids.remove(0);
                self.asks.remove(0);
            } else {
                break;
            }
        }
    }
}

fn benchmark_matching(c: &mut Criterion) {
    c.bench_function ("match 100 orders", |b| {
        b.iter(|| {
            let mut book = OrderBook::new();
            for i in 0..50 {
                book.add_order(Order {
                    symbol: String::from("NVDA"),
                    price: 500.0 -i as f64,
                    quantity: 10,
                    side: String::from("BUY"),
                });
                book.add_order(Order {
                    symbol: String::from("NVDA"),
                    price: 490.0 + i as f64,
                    quantity: 10,
                    side: String::from("SELL"),
                });
            }
            book.match_orders();
        })
    });
}

criterion_group!(benches, benchmark_matching);
criterion_main!(benches);