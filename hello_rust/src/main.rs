#[derive(PartialEq)]
enum OrderSide{
    Buy,
    Sell,
}
#[derive(Debug)]
enum OrderResult {
    Filled(u32),
    PartialFill(u32, u32),
    Rejected(String),
}
struct Order {
    id: u32,
    symbol: String,
    price: f64,
    quantity: u32,
    side: OrderSide,
}
impl Order {
    fn new(id: u32, symbol: &str, price: f64, quantity: u32, side: OrderSide) -> Order{
        Order {
            id,
            symbol: String::from(symbol),
            price,
            quantity,
            side,
        }
    }
}
struct OrderBook {
    orders: Vec<Order>,
}

impl OrderBook {
    fn new() -> OrderBook {
        OrderBook {
            orders: Vec::new(),
        }
    }

    fn print_book(&self) {
        println!("---- ORDER BOOK ----");
        for order in &self.orders {
            let side = match &order.side {
                OrderSide::Buy  => "BUY",
                OrderSide::Sell => "SELL",
            };
            println!("{} | {} | {} units at ${}",
                order.id, side, order.quantity, order.price);
        }
        println!("--- END ---");
    }

    fn add_order(&mut self, order: Order) {
        println!("Adding order {} for {} units at ${}", order.id, order.quantity, order.price);
        self.orders.push(order);
    }

    fn total_orders(&self) -> usize {
        self.orders.len()
    }

    fn match_orders(&mut self) -> Vec<OrderResult> {
        let mut results = Vec::new();
        let mut i = 0;
        while i < self.orders.len() {
            let mut j = i + 1;
            while j < self.orders.len() {
                let matched = {
                    let buy  = &self.orders[i];
                    let sell = &self.orders[j];
                    buy.symbol == sell.symbol &&
                    buy.price  >= sell.price &&
                    buy.side == OrderSide::Buy &&
                    sell.side == OrderSide::Sell
                };
                if matched {
                    let trade_qty = self.orders[i].quantity.min(self.orders[j].quantity);
                    println!("MATCH: {} units of {} at ${}",
                        trade_qty,
                        self.orders[i].symbol,
                        self.orders[j].price);
                    self.orders[i].quantity -= trade_qty;
                    self.orders[j].quantity -= trade_qty;
                    
                    if self.orders[i].quantity == 0 {
                        results.push(OrderResult::Filled(trade_qty));
                    } else {
                        results.push(OrderResult::PartialFill(trade_qty, self.orders[i].quantity));
                    }
                    if self.orders[j].quantity == 0 {
                        self.orders.remove(j);
                    } else {
                        j += 1;
                    }
                
                } else {
                    j+=1;
                }
            }
            if self.orders[i].quantity == 0 {
                self.orders.remove(i);
            } else {
                i += 1;
            }
        }
        results
    }
}

fn main() {
    let mut book = OrderBook::new();
    
    book.add_order(Order::new(1, "NVDA", 500.0, 10, OrderSide::Buy));
    book.add_order(Order::new(2, "NVDA", 495.0, 5,  OrderSide::Sell));
    book.add_order(Order::new(3, "AAPL", 150.0, 20, OrderSide::Buy));
    
    let results = book.match_orders();
    for result in &results {
        println!("Result: {:?}", result);
    }
    
    println!("Total orders in book: {}", book.total_orders());
    book.print_book();
}