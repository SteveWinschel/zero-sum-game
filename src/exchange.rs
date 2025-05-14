use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap, VecDeque}; // BTreeMap for sorted order book
use uuid::Uuid;

use crate::order::{Order, OrderType, Trade};
use crate::securities_lib::{Industry, Security};
use crate::trader::Trader; // Assuming direct interaction, might be event-based

// Order book for a single security
// We use BTreeMap to keep orders sorted by price.
// For Buy orders: Higher price is better (so descending order of price).
// For Sell orders: Lower price is better (so ascending order of price).
// Within the same price, orders are typically FIFO (First-In, First-Out), hence VecDeque.
type BuyOrderBook = BTreeMap<u64, VecDeque<Order>>; // Price (desc) -> Queue of Orders
type SellOrderBook = BTreeMap<u64, VecDeque<Order>>; // Price (asc) -> Queue of Orders

pub struct OrderBook {
    pub buy_orders: BuyOrderBook, // Max-heap based on price (BTreeMap sorts keys)
    pub sell_orders: SellOrderBook, // Min-heap based on price (BTreeMap sorts keys)
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            buy_orders: BTreeMap::new(), // Use std::collections::btree_map::Keys::rev() for descending iteration
            sell_orders: BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        match order.order_type {
            OrderType::Buy => {
                // For BTreeMap, higher price means it comes later. To get FIFO for buys (highest price first),
                // we store price and iterate in reverse or store negative price if we wanted strict heap behavior.
                // Simpler: BTreeMap sorts ascendingly. For buys, we want to match highest bid.
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
    pub securities: HashMap<Cow<'static, str>, Security>, // Ticker -> Security details
    order_books: HashMap<Cow<'static, str>, OrderBook>,   // Ticker -> OrderBook
    // traders: HashMap<Uuid, Trader>, // If the exchange manages traders directly
    // This is a simple model. A real exchange would have more complex trader/member management.
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

    // This is a simplified order processing logic.
    // A real system would need to handle partial fills, different order types (market, limit, etc.),
    // and potentially update trader portfolios directly or via an event system.
    pub fn place_order(&mut self, order: Order) -> Result<Vec<Trade>, String> {
        if !self.securities.contains_key(&order.security_ticker)
            || !self.securities[&order.security_ticker].tradable
        {
            return Err(format!(
                "Security {} is not tradable or does not exist.",
                order.security_ticker
            ));
        }

        let order_book = self.order_books.get_mut(&order.security_ticker).unwrap(); // Should exist if security exists
        let mut trades = Vec::new();
        let mut order_to_process = order.clone(); // Clone order to modify its quantity

        match order_to_process.order_type {
            OrderType::Buy => {
                // Iterate sell orders from lowest price (best for buyer)
                let mut prices_to_remove_from_sell_book = Vec::new();
                for (&sell_price, sell_orders_at_price) in order_book.sell_orders.iter_mut() {
                    if order_to_process.quantity == 0 {
                        break;
                    } // Buy order filled
                    if sell_price > order_to_process.price {
                        break;
                    } // No more sell orders at or below buy limit

                    let mut orders_in_queue_to_remove = 0;
                    for sell_order in sell_orders_at_price.iter_mut() {
                        if order_to_process.quantity == 0 {
                            break;
                        }

                        let trade_quantity =
                            std::cmp::min(order_to_process.quantity, sell_order.quantity);
                        let trade_price = sell_price; // Trade occurs at the existing order's price (sell_order's price)

                        let trade = Trade::new(
                            order_to_process.id,
                            sell_order.id,
                            order_to_process.security_ticker.clone(),
                            trade_quantity,
                            trade_price,
                            0, // Placeholder for actual trade timestamp
                        );
                        trades.push(trade.clone());
                        self.trade_history.push(trade);

                        order_to_process.quantity -= trade_quantity;
                        sell_order.quantity -= trade_quantity;

                        // Update security's last traded price and volume
                        if let Some(sec) =
                            self.securities.get_mut(&order_to_process.security_ticker)
                        {
                            sec.record_trade(trade_price, trade_quantity);
                        }

                        if sell_order.quantity == 0 {
                            orders_in_queue_to_remove += 1;
                        }
                    }
                    // Remove filled orders from the queue
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

                // If buy order is not fully filled, add remaining to buy order book
                if order_to_process.quantity > 0 {
                    order_book.add_order(order_to_process);
                }
            }
            OrderType::Sell => {
                // Iterate buy orders from highest price (best for seller) - BTreeMap iterates ascendingly by default
                let mut prices_to_remove_from_buy_book = Vec::new();
                // We need to iterate in reverse for buy orders (highest price first)
                for (&buy_price, buy_orders_at_price) in order_book.buy_orders.iter_mut().rev() {
                    if order_to_process.quantity == 0 {
                        break;
                    } // Sell order filled
                    if buy_price < order_to_process.price {
                        break;
                    } // No more buy orders at or above sell limit

                    let mut orders_in_queue_to_remove = 0;
                    for buy_order in buy_orders_at_price.iter_mut() {
                        if order_to_process.quantity == 0 {
                            break;
                        }

                        let trade_quantity =
                            std::cmp::min(order_to_process.quantity, buy_order.quantity);
                        let trade_price = buy_price; // Trade occurs at the existing order's price (buy_order's price)

                        let trade = Trade::new(
                            buy_order.id,
                            order_to_process.id,
                            order_to_process.security_ticker.clone(),
                            trade_quantity,
                            trade_price,
                            0, // Placeholder
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

                // If sell order is not fully filled, add remaining to sell order book
                if order_to_process.quantity > 0 {
                    order_book.add_order(order_to_process);
                }
            }
        }
        Ok(trades)
    }

    pub fn get_order_book_display(&self, ticker: &str) -> Option<String> {
        self.order_books.get(ticker).map(|ob| {
            let mut display = String::new();
            display.push_str(&format!("Order Book for {}:\n", ticker));
            display.push_str("  --- SELL ORDERS (Asks) ---\n");
            for (price, orders) in ob.sell_orders.iter() {
                // Lowest price first
                for order in orders {
                    display.push_str(&format!(
                        "    Price: ${:.2}, Qty: {}\n",
                        *price as f64 / 100.0,
                        order.quantity
                    ));
                }
            }
            display.push_str("  --- BUY ORDERS (Bids) ---\n");
            for (price, orders) in ob.buy_orders.iter().rev() {
                // Highest price first
                for order in orders {
                    display.push_str(&format!(
                        "    Price: ${:.2}, Qty: {}\n",
                        *price as f64 / 100.0,
                        order.quantity
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
            for trade in &self.trade_history {
                display.push_str(&format!(
                    "  Ticker: {}, Qty: {}, Price: ${:.2}, BuyO_ID: ..., SellO_ID: ...\n",
                    trade.security_ticker,
                    trade.quantity,
                    trade.price as f64 / 100.0,
                    // trade.buy_order_id, // Too verbose for simple display
                    // trade.sell_order_id,
                ));
            }
        }
        display
    }
}
