use std::borrow::Cow;
use std::fmt;
use uuid::Uuid; // For unique order IDs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Buy,
    Sell,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: Uuid,        // Unique identifier for the order
    pub trader_id: Uuid, // ID of the trader placing the order
    pub order_type: OrderType,
    pub security_ticker: Cow<'static, str>, // Ticker symbol of the security
    pub quantity: u64,                      // Number of shares
    pub price: u64,                         // Price per share in cents (limit price)
    pub timestamp: u64, // Order creation timestamp (e.g., nanoseconds since epoch)
                        // You might want to use a proper datetime library like `chrono`
}

impl Order {
    pub fn new(
        trader_id: Uuid,
        order_type: OrderType,
        security_ticker: &'static str,
        quantity: u64,
        price: u64,
    ) -> Self {
        // For timestamp, you'd normally use something like:
        // std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64
        // For simplicity in this example, we'll use 0 or allow it to be passed.
        Order {
            id: Uuid::new_v4(),
            trader_id,
            order_type,
            security_ticker: Cow::Borrowed(security_ticker),
            quantity,
            price,
            timestamp: 0, // Replace with actual timestamp logic
        }
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Order ID: {}", self.id)?;
        writeln!(f, "  Trader ID: {}", self.trader_id)?;
        writeln!(f, "  Type: {}", self.order_type)?;
        writeln!(f, "  Ticker: {}", self.security_ticker)?;
        writeln!(f, "  Quantity: {}", self.quantity)?;
        writeln!(f, "  Price: ${:.2}", self.price as f64 / 100.0)?;
        writeln!(f, "  Timestamp: {}", self.timestamp) // Format timestamp appropriately
    }
}

// You might also define a Trade struct here to represent executed trades
#[derive(Debug, Clone)]
pub struct Trade {
    pub buy_order_id: Uuid,
    pub sell_order_id: Uuid,
    pub security_ticker: Cow<'static, str>,
    pub quantity: u64,
    pub price: u64, // Execution price
    pub timestamp: u64,
}

impl Trade {
    pub fn new(
        buy_order_id: Uuid,
        sell_order_id: Uuid,
        security_ticker: Cow<'static, str>,
        quantity: u64,
        price: u64,
        timestamp: u64,
    ) -> Self {
        Trade {
            buy_order_id,
            sell_order_id,
            security_ticker,
            quantity,
            price,
            timestamp,
        }
    }
}
