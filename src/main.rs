use uuid::Uuid; // Make sure uuid is a dependency in Cargo.toml

// Declare modules
mod exchange;
mod order;
mod securities;
mod securities_lib;
mod trader;

// Use necessary items
use exchange::StockExchange;
use order::{Order, OrderType};
use securities::get_initial_securities;
use securities_lib::{Industry, Security};
use trader::Trader;

fn main() {
    println!("--- Zero Sum Game Stock Exchange Simulation ---");

    // 1. Create the stock exchange
    let mut stock_exchange = StockExchange::new();
    println!("\nStock Exchange created.");

    // 2. Add securities to the exchange
    let initial_securities = get_initial_securities();
    for sec in initial_securities {
        println!("Adding security: {}", sec.ticker_symbol);
        if let Err(e) = stock_exchange.add_security(sec) {
            eprintln!("Error adding security: {}", e);
        }
    }

    println!("\n--- Initial Securities in Exchange ---");
    for ticker in stock_exchange.securities.keys() {
        if let Some(sec_detail) = stock_exchange.get_security(ticker) {
            println!("{}", sec_detail);
        }
    }

    // 3. Create some traders
    let mut trader1 = Trader::new("Alice".to_string(), 1000000); // $10,000.00
    let mut trader2 = Trader::new("Bob".to_string(), 1500000); // $15,000.00
    println!("\nTraders created:");
    println!(
        "  {} (ID: {}) with cash: ${:.2}",
        trader1.name,
        trader1.id,
        trader1.cash_balance as f64 / 100.0
    );
    println!(
        "  {} (ID: {}) with cash: ${:.2}",
        trader2.name,
        trader2.id,
        trader2.cash_balance as f64 / 100.0
    );

    // 4. Simulate some orders and trades
    // (For simplicity, we are not yet deducting cash or adding shares to trader portfolios upon order placement;
    // this would happen upon trade execution, which requires more complex state management for traders
    // within the exchange or an event system.)

    println!("\n--- Placing Orders ---");

    // Alice wants to buy 10 XYZ at $150.00
    let alice_buy_order = Order::new(trader1.id, OrderType::Buy, "XYZ", 10, 15000);
    println!(
        "Alice places BUY order: {} {} @ ${:.2}",
        alice_buy_order.quantity,
        alice_buy_order.security_ticker,
        alice_buy_order.price as f64 / 100.0
    );
    match stock_exchange.place_order(alice_buy_order.clone()) {
        Ok(trades) => {
            if trades.is_empty() {
                println!("  Order added to book.");
            }
            for trade in trades {
                println!(
                    "  TRADE EXECUTED: {} {} @ ${:.2}",
                    trade.quantity,
                    trade.security_ticker,
                    trade.price as f64 / 100.0
                );
            }
        }
        Err(e) => eprintln!("  Error placing order: {}", e),
    }

    // Bob wants to sell 5 XYZ at $150.00
    let bob_sell_order = Order::new(trader2.id, OrderType::Sell, "XYZ", 5, 15000);
    println!(
        "Bob places SELL order: {} {} @ ${:.2}",
        bob_sell_order.quantity,
        bob_sell_order.security_ticker,
        bob_sell_order.price as f64 / 100.0
    );
    match stock_exchange.place_order(bob_sell_order.clone()) {
        Ok(trades) => {
            if trades.is_empty() {
                println!("  Order added to book.");
            }
            // Here you would update trader1 and trader2 portfolios and cash based on executed trades
            for trade in &trades {
                println!(
                    "  TRADE EXECUTED: {} {} @ ${:.2}",
                    trade.quantity,
                    trade.security_ticker,
                    trade.price as f64 / 100.0
                );
                // Example: Update Alice's portfolio (buyer)
                trader1.cash_balance -= trade.quantity * trade.price;
                trader1.add_shares(&trade.security_ticker, trade.quantity, trade.price);

                // Example: Update Bob's portfolio (seller)
                trader2.cash_balance += trade.quantity * trade.price;
                trader2
                    .remove_shares(&trade.security_ticker, trade.quantity)
                    .unwrap(); // Assuming it succeeds
            }
        }
        Err(e) => eprintln!("  Error placing order: {}", e),
    }

    // Bob wants to sell 10 ABC at $75.00
    let bob_sell_order_abc = Order::new(trader2.id, OrderType::Sell, "ABC", 10, 7500);
    println!(
        "Bob places SELL order: {} {} @ ${:.2}",
        bob_sell_order_abc.quantity,
        bob_sell_order_abc.security_ticker,
        bob_sell_order_abc.price as f64 / 100.0
    );
    match stock_exchange.place_order(bob_sell_order_abc) {
        Ok(trades) => {
            if trades.is_empty() {
                println!("  Order added to book.");
            }
            for trade in trades {
                println!(
                    "  TRADE EXECUTED: {} {} @ ${:.2}",
                    trade.quantity,
                    trade.security_ticker,
                    trade.price as f64 / 100.0
                );
            }
        }
        Err(e) => eprintln!("  Error placing order: {}", e),
    }

    // Alice wants to buy 20 ABC at $76.00
    let alice_buy_order_abc = Order::new(trader1.id, OrderType::Buy, "ABC", 20, 7600);
    println!(
        "Alice places BUY order: {} {} @ ${:.2}",
        alice_buy_order_abc.quantity,
        alice_buy_order_abc.security_ticker,
        alice_buy_order_abc.price as f64 / 100.0
    );
    match stock_exchange.place_order(alice_buy_order_abc) {
        Ok(trades) => {
            if trades.is_empty() {
                println!("  Order added to book.");
            }
            for trade in &trades {
                println!(
                    "  TRADE EXECUTED: {} {} @ ${:.2}",
                    trade.quantity,
                    trade.security_ticker,
                    trade.price as f64 / 100.0
                );
                trader1.cash_balance -= trade.quantity * trade.price;
                trader1.add_shares(&trade.security_ticker, trade.quantity, trade.price);

                trader2.cash_balance += trade.quantity * trade.price;
                trader2
                    .remove_shares(&trade.security_ticker, trade.quantity)
                    .unwrap();
            }
        }
        Err(e) => eprintln!("  Error placing order: {}", e),
    }

    println!("\n--- Current Order Books ---");
    for ticker in stock_exchange.securities.keys() {
        if let Some(book_display) = stock_exchange.get_order_book_display(ticker) {
            println!("{}", book_display);
        }
    }

    println!("\n{}", stock_exchange.get_trade_history_display());

    println!("\n--- Final Security Status ---");
    for ticker in stock_exchange.securities.keys() {
        if let Some(sec_detail) = stock_exchange.get_security(ticker) {
            println!("{}", sec_detail);
        }
    }

    println!("\n--- Final Trader Status ---");
    println!(
        "  Alice: Cash ${:.2}, Portfolio: {:?}",
        trader1.cash_balance as f64 / 100.0,
        trader1.portfolio
    );
    println!(
        "  Bob:   Cash ${:.2}, Portfolio: {:?}",
        trader2.cash_balance as f64 / 100.0,
        trader2.portfolio
    );

    // Get current market prices for portfolio valuation
    let mut market_prices = std::collections::HashMap::new();
    for (ticker, security) in &stock_exchange.securities {
        market_prices.insert(ticker.clone(), security.current_price);
    }
    println!(
        "  Alice Net Worth: ${:.2}",
        trader1.get_total_net_worth(&market_prices) as f64 / 100.0
    );
    println!(
        "  Bob Net Worth: ${:.2}",
        trader2.get_total_net_worth(&market_prices) as f64 / 100.0
    );
}
