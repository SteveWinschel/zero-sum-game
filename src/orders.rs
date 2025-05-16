use std::borrow::Cow;
use std::fmt;

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
    pub id: u64, // Changed from Uuid to u64
    pub order_type: OrderType,
    pub security_ticker: Cow<'static, str>,
    pub quantity: u64,
    pub price: u64,
    pub timestamp: u64, // Consider using a proper datetime library or a simple counter for sequence
}

impl Order {
    pub fn new(
        id: u64, // Added id as a parameter
        order_type: OrderType,
        security_ticker: &'static str,
        quantity: u64,
        price: u64,
        timestamp: u64, // Added timestamp as a parameter
    ) -> Self {
        Order {
            id,
            order_type,
            security_ticker: Cow::Borrowed(security_ticker),
            quantity,
            price,
            timestamp,
        }
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Order ID: {}", self.id)?; // Will now display u64
        writeln!(f, "  Type: {}", self.order_type)?;
        writeln!(f, "  Ticker: {}", self.security_ticker)?;
        writeln!(f, "  Quantity: {}", self.quantity)?;
        writeln!(f, "  Price: ${:.2}", self.price as f64 / 100.0)?;
        writeln!(f, "  Timestamp: {}", self.timestamp)
    }
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub matched_buy_order_id: u64,
    pub matched_sell_order_id: u64,
    pub security_ticker: Cow<'static, str>,
    pub quantity: u64,
    pub price: u64, // Execution price
    pub timestamp: u64,
}

impl Trade {
    pub fn new(
        matched_buy_order_id: u64,
        matched_sell_order_id: u64,
        security_ticker: Cow<'static, str>,
        quantity: u64,
        price: u64,
        timestamp: u64,
    ) -> Self {
        Trade {
            matched_buy_order_id,
            matched_sell_order_id,
            security_ticker,
            quantity,
            price,
            timestamp,
        }
    }
}
