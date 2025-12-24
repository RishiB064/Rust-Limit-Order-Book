use rand::Rng;
use std::collections::{BTreeMap, VecDeque};
use std::time::Instant;

// --- 1. DATA STRUCTURES ---

#[derive(Debug, Clone, Copy, PartialEq)]
enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
struct Order {
    id: u64,
    order_type: OrderType,
   
    price: u64,
    quantity: u32,
}

struct OrderBook {
    // Bids (Buyers): We want the HIGHEST price first.
    // Asks (Sellers): We want the LOWEST price first.
    // BTreeMap sorts Low -> High automatically.
    bids: BTreeMap<u64, VecDeque<Order>>,
    asks: BTreeMap<u64, VecDeque<Order>>,
}

impl OrderBook {
    fn new() -> OrderBook {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    fn add_order(&mut self, mut order: Order) {
        match order.order_type {
            OrderType::Buy => self.match_buy_order(&mut order),
            OrderType::Sell => self.match_sell_order(&mut order),
        }
    }

    // --- 2. MATCHING LOGIC (The Engine) ---

    fn match_buy_order(&mut self, order: &mut Order) {
      
        while order.quantity > 0 {
            // 1. PEEK: Look at the cheapest seller (First key in BTreeMap)
            
            let best_ask_price = if let Some(&price) = self.asks.keys().next() {
                price
            } else {
                break;
            };

            // 2. CHECK: Is the seller too expensive?
            if best_ask_price > order.price {
                break;
            }

            // 3. MATCH: Get the queue of sellers at this price
            let orders_at_level = self.asks.get_mut(&best_ask_price).unwrap();

            // (FIFO - Time Priority)
            while order.quantity > 0 && !orders_at_level.is_empty() {
                let best_ask = orders_at_level.front_mut().unwrap();

                
                let trade_qty = std::cmp::min(order.quantity, best_ask.quantity);

                // Execute Trade 
                order.quantity -= trade_qty;
                best_ask.quantity -= trade_qty;

                // If seller is empty, remove them
                if best_ask.quantity == 0 {
                    orders_at_level.pop_front();
                }
            }

            // If no sellers left at this price, remove the price level
            if orders_at_level.is_empty() {
                self.asks.remove(&best_ask_price);
            }
        }

        // 4. PARK: If order is not filled, put it in the book
        if order.quantity > 0 {
            self.bids
                .entry(order.price)
                .or_insert_with(VecDeque::new)
                .push_back(order.clone());
        }
    }

    fn match_sell_order(&mut self, order: &mut Order) {
       
        while order.quantity > 0 {
            // 1. PEEK: Look at highest bidder (Last key in BTreeMap)
            // .keys().next_back() grabs the end of the map
            let best_bid_price = if let Some(&price) = self.bids.keys().next_back() {
                price
            } else {
                break;
            };

            // 2. CHECK: Is the buyer offering enough?
            if best_bid_price < order.price {
                break;
            }

            // 3. MATCH
            let orders_at_level = self.bids.get_mut(&best_bid_price).unwrap();

            while order.quantity > 0 && !orders_at_level.is_empty() {
                let best_bid = orders_at_level.front_mut().unwrap();
                let trade_qty = std::cmp::min(order.quantity, best_bid.quantity);

                order.quantity -= trade_qty;
                best_bid.quantity -= trade_qty;

                if best_bid.quantity == 0 {
                    orders_at_level.pop_front();
                }
            }

            if orders_at_level.is_empty() {
                self.bids.remove(&best_bid_price);
            }
        }

        // 4. PARK
        if order.quantity > 0 {
            self.asks
                .entry(order.price)
                .or_insert_with(VecDeque::new)
                .push_back(order.clone());
        }
    }
}



fn main() {
    let mut book = OrderBook::new();
    let mut rng = rand::thread_rng();
    let total_orders = 1_000_000;

    println!(" INITIALIZING HIGH-FREQUENCY ENGINE...");
    println!("Target: Process {} Orders", total_orders);

  
    let start_time = Instant::now();

    for i in 0..total_orders {
      ]
        let is_buy = rng.gen_bool(0.5);
        let price = rng.gen_range(9000..11000); 
        let qty = rng.gen_range(1..100);

        let order = Order {
            id: i,
            order_type: if is_buy {
                OrderType::Buy
            } else {
                OrderType::Sell
            },
            price,
            quantity: qty,
        };

        book.add_order(order);
    }

    let duration = start_time.elapsed();

    // --- REPORTING ---
    let seconds = duration.as_secs_f64();
    let ops = total_orders as f64 / seconds;
    let latency_ns = (seconds / total_orders as f64) * 1_000_000_000.0;

    println!("\n DONE.");
    println!("---------------------------------------------");
    println!("Time Taken:      {:.4} seconds", seconds);
    println!("Throughput:      {:.0} Orders/Sec", ops);
    println!("Latency per Order: {:.0} nanoseconds", latency_ns);
    println!("---------------------------------------------");
}
