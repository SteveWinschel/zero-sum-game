use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap, VecDeque};

use crate::orders::{Order, OrderType, Trade};
use crate::securities_lib::{Security, Trend}; // Added Trend

// ... (OrderBook struct and impl remains the same)
type BuyOrderBook = BTreeMap<u64, VecDeque<Order>>;
type SellOrderBook = BTreeMap<u64, VecDeque<Order>>;

pub struct OrderBook {
    pub buy_orders: BuyOrderBook,
    pub sell_orders: SellOrderBook,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            buy_orders: BTreeMap::new(),
            sell_orders: BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        match order.order_type {
            OrderType::Buy => {
                self.buy_orders
                    .entry(order.price)
                    .or_default()
                    .push_back(order);
            }
            OrderType::Sell => {
                self.sell_orders
                    .entry(order.price)
                    .or_default()
                    .push_back(order);
            }
        }
    }
}

pub struct StockExchange {
    pub securities: HashMap<Cow<'static, str>, Security>,
    order_books: HashMap<Cow<'static, str>, OrderBook>,
    trade_history: Vec<Trade>,
}

impl StockExchange {
    pub fn new() -> Self {
        StockExchange {
            securities: HashMap::new(),
            order_books: HashMap::new(),
            trade_history: Vec::new(),
        }
    }

    pub fn add_security(&mut self, security: Security) -> Result<(), String> {
        if self.securities.contains_key(&security.ticker_symbol) {
            return Err(format!(
                "Security with ticker {} already exists.",
                security.ticker_symbol
            ));
        }
        let ticker = security.ticker_symbol.clone();
        self.securities.insert(ticker.clone(), security);
        self.order_books.insert(ticker, OrderBook::new());
        Ok(())
    }

    pub fn get_security(&self, ticker: &str) -> Option<&Security> {
        self.securities.get(ticker)
    }

    pub fn get_security_mut(&mut self, ticker: &str) -> Option<&mut Security> {
        self.securities.get_mut(ticker)
    }

    pub fn place_order(&mut self, order: Order) -> Result<Vec<Trade>, String> {
        let security_exists_and_tradable = self
            .securities
            .get(&order.security_ticker)
            .map_or(false, |sec| sec.trend != Trend::Banned); // Updated logic

        if !security_exists_and_tradable {
            return Err(format!(
                "Security {} is not tradable (current trend is Banned) or does not exist.",
                order.security_ticker
            ));
        }

        let trade_timestamp = order.timestamp;

        let order_book = self.order_books.get_mut(&order.security_ticker).unwrap();
        let mut trades = Vec::new();
        let mut order_to_process = order.clone();

        match order_to_process.order_type {
            OrderType::Buy => {
                // ... (rest of the Buy logic remains the same)
                let mut prices_to_remove_from_sell_book = Vec::new();
                for (&sell_price, sell_orders_at_price) in order_book.sell_orders.iter_mut() {
                    if order_to_process.quantity == 0 {
                        break;
                    }
                    if sell_price > order_to_process.price {
                        break;
                    }

                    let mut orders_in_queue_to_remove = 0;
                    for sell_order in sell_orders_at_price.iter_mut() {
                        if order_to_process.quantity == 0 {
                            break;
                        }

                        let trade_quantity =
                            std::cmp::min(order_to_process.quantity, sell_order.quantity);
                        let trade_price = sell_price;

                        let trade = Trade::new(
                            order_to_process.id,
                            sell_order.id,
                            order_to_process.security_ticker.clone(),
                            trade_quantity,
                            trade_price,
                            trade_timestamp,
                        );
                        trades.push(trade.clone());
                        self.trade_history.push(trade);

                        order_to_process.quantity -= trade_quantity;
                        sell_order.quantity -= trade_quantity;

                        if let Some(sec) =
                            self.securities.get_mut(&order_to_process.security_ticker)
                        {
                            sec.record_trade(trade_price, trade_quantity);
                        }

                        if sell_order.quantity == 0 {
                            orders_in_queue_to_remove += 1;
                        }
                    }
                    for _ in 0..orders_in_queue_to_remove {
                        sell_orders_at_price.pop_front();
                    }
                    if sell_orders_at_price.is_empty() {
                        prices_to_remove_from_sell_book.push(sell_price);
                    }
                }
                for price_key in prices_to_remove_from_sell_book {
                    order_book.sell_orders.remove(&price_key);
                }

                if order_to_process.quantity > 0 {
                    order_book.add_order(order_to_process);
                }
            }
            OrderType::Sell => {
                // ... (rest of the Sell logic remains the same)
                let mut prices_to_remove_from_buy_book = Vec::new();
                for (&buy_price, buy_orders_at_price) in order_book.buy_orders.iter_mut().rev() {
                    if order_to_process.quantity == 0 {
                        break;
                    }
                    if buy_price < order_to_process.price {
                        break;
                    }

                    let mut orders_in_queue_to_remove = 0;
                    for buy_order in buy_orders_at_price.iter_mut() {
                        if order_to_process.quantity == 0 {
                            break;
                        }

                        let trade_quantity =
                            std::cmp::min(order_to_process.quantity, buy_order.quantity);
                        let trade_price = buy_price;

                        let trade = Trade::new(
                            buy_order.id,
                            order_to_process.id,
                            order_to_process.security_ticker.clone(),
                            trade_quantity,
                            trade_price,
                            trade_timestamp,
                        );
                        trades.push(trade.clone());
                        self.trade_history.push(trade);

                        order_to_process.quantity -= trade_quantity;
                        buy_order.quantity -= trade_quantity;

                        if let Some(sec) =
                            self.securities.get_mut(&order_to_process.security_ticker)
                        {
                            sec.record_trade(trade_price, trade_quantity);
                        }

                        if buy_order.quantity == 0 {
                            orders_in_queue_to_remove += 1;
                        }
                    }
                    for _ in 0..orders_in_queue_to_remove {
                        buy_orders_at_price.pop_front();
                    }
                    if buy_orders_at_price.is_empty() {
                        prices_to_remove_from_buy_book.push(buy_price);
                    }
                }
                for price_key in prices_to_remove_from_buy_book {
                    order_book.buy_orders.remove(&price_key);
                }

                if order_to_process.quantity > 0 {
                    order_book.add_order(order_to_process);
                }
            }
        }
        Ok(trades)
    }

    // ... (get_order_book_display and get_trade_history_display remain the same)
    pub fn get_order_book_display(&self, ticker: &str) -> Option<String> {
        self.order_books.get(ticker).map(|ob| {
            let mut display = String::new();
            display.push_str(&format!("Order Book for {}:\n", ticker));
            display.push_str("  --- SELL ORDERS (Asks) ---\n");
            for (price, orders) in ob.sell_orders.iter() {
                for order in orders {
                    display.push_str(&format!(
                        "    Price: ${:.2}, Qty: {}, ID: {}\n",
                        *price as f64 / 100.0,
                        order.quantity,
                        order.id
                    ));
                }
            }
            display.push_str("  --- BUY ORDERS (Bids) ---\n");
            for (price, orders) in ob.buy_orders.iter().rev() {
                for order in orders {
                    display.push_str(&format!(
                        "    Price: ${:.2}, Qty: {}, ID: {}\n",
                        *price as f64 / 100.0,
                        order.quantity,
                        order.id
                    ));
                }
            }
            display
        })
    }

    pub fn get_trade_history_display(&self) -> String {
        let mut display = String::from("--- TRADE HISTORY ---\n");
        if self.trade_history.is_empty() {
            display.push_str("No trades yet.\n");
        } else {
            for (index, trade) in self.trade_history.iter().enumerate() {
                display.push_str(&format!(
                    "  Trade Index: {}, Ticker: {}, Qty: {}, Price: ${:.2}, Timestamp: {}\n",
                    index,
                    trade.security_ticker,
                    trade.quantity,
                    trade.price as f64 / 100.0,
                    trade.timestamp,
                ));
                display.push_str(&format!(
                    "    Matched Buy Order ID: {}, Matched Sell Order ID: {}\n",
                    trade.matched_buy_order_id, trade.matched_sell_order_id,
                ));
            }
        }
        display
    }
}
