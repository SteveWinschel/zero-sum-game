mod exchange;
mod orders;
mod player;
mod securities;
mod securities_lib;

// Use necessary items
use exchange::StockExchange;
use orders::{Order, OrderType, Trade}; // Trade might not be directly used here after changes
use player::Player;
use securities::get_initial_securities;

// For simplistic timestamping, you can use a counter or a more elaborate time source.
// For now, we'll use a simple counter for timestamps as well, or just 0.
fn get_current_timestamp() -> u64 {
    // In a real application, use std::time::SystemTime or a crate like chrono.
    // For this simulation, a simple incrementing counter or 0 is fine if precise timing isn't critical.
    // Let's use 0 for now for simplicity in this example, or pass it from a counter.
    0 // Placeholder timestamp
}

fn main() {
    println!("--- Zero Sum Game Stock Exchange Simulation ---");

    let mut next_order_id_counter: u64 = 1;
    let mut current_sim_time: u64 = 0; // Simple time counter for orders/trades

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

    // 3. Create the single player
    let mut player = Player::new("Player 1".to_string(), 1000000); // $10,000.00
    println!("\nPlayer created:");
    println!(
        "  {} with cash: ${:.2}",
        player.name,
        player.cash_balance as f64 / 100.0
    );

    println!("\n--- Placing Orders ---");

    // Player wants to buy 10 XYZ at $150.00
    current_sim_time += 1;
    let buy_order_xyz = Order::new(
        next_order_id_counter,
        OrderType::Buy,
        "XYZ",
        10,
        15000,
        current_sim_time,
    );
    next_order_id_counter += 1;
    println!(
        "{} places BUY order (ID: {}): {} {} @ ${:.2}",
        player.name,
        buy_order_xyz.id,
        buy_order_xyz.quantity,
        buy_order_xyz.security_ticker,
        buy_order_xyz.price as f64 / 100.0
    );
    match stock_exchange.place_order(buy_order_xyz.clone()) {
        Ok(trades) => {
            if trades.is_empty() {
                println!("  Order added to book.");
            }
            for trade in trades {
                println!(
                    "  TRADE EXECUTED: {} {} @ ${:.2} (Buy Order: {}, Sell Order: {})",
                    trade.quantity,
                    trade.security_ticker,
                    trade.price as f64 / 100.0,
                    trade.matched_buy_order_id,
                    trade.matched_sell_order_id
                );
                if trade.matched_buy_order_id == buy_order_xyz.id {
                    // Check if this specific order was involved
                    player.cash_balance -= trade.quantity * trade.price;
                    player.add_shares(&trade.security_ticker, trade.quantity, trade.price);
                }
            }
        }
        Err(e) => eprintln!("  Error placing order: {}", e),
    }

    // Market places a sell order to match the player's buy order
    current_sim_time += 1;
    let market_sell_order_xyz = Order::new(
        next_order_id_counter,
        OrderType::Sell,
        "XYZ",
        5,
        15000,
        current_sim_time,
    );
    next_order_id_counter += 1;
    println!(
        "Market places SELL order (ID: {}): {} {} @ ${:.2}",
        market_sell_order_xyz.id,
        market_sell_order_xyz.quantity,
        market_sell_order_xyz.security_ticker,
        market_sell_order_xyz.price as f64 / 100.0
    );
    match stock_exchange.place_order(market_sell_order_xyz.clone()) {
        Ok(trades) => {
            if trades.is_empty() {
                println!("  Order added to book.");
            }
            for trade in &trades {
                println!(
                    "  TRADE EXECUTED (market): {} {} @ ${:.2} (Buy Order: {}, Sell Order: {})",
                    trade.quantity,
                    trade.security_ticker,
                    trade.price as f64 / 100.0,
                    trade.matched_buy_order_id,
                    trade.matched_sell_order_id
                );
                if trade.matched_buy_order_id == buy_order_xyz.id {
                    // Player was the buyer, cash should have been deducted when their buy order got matched.
                    // If not already handled above (it was), this is where you'd ensure it.
                    // Re-checking logic: player's portfolio is updated when THEIR order leads to a trade.
                    // The previous block already handles this if buy_order_xyz is matched.
                    println!(
                        "    Player's buy order (ID: {}) part of this trade.",
                        buy_order_xyz.id
                    );
                }
            }
        }
        Err(e) => eprintln!("  Error placing order: {}", e),
    }

    // Player acquires some ABC shares to sell later
    player.add_shares("ABC", 20, 7400);
    player.cash_balance -= 20 * 7400;
    println!(
        "\n{} manually acquired 20 ABC for testing. Current cash: ${:.2}",
        player.name,
        player.cash_balance as f64 / 100.0
    );

    // Player wants to sell 10 ABC at $75.00
    current_sim_time += 1;
    let player_sell_order_abc = Order::new(
        next_order_id_counter,
        OrderType::Sell,
        "ABC",
        10,
        7500,
        current_sim_time,
    );
    next_order_id_counter += 1;
    println!(
        "{} places SELL order (ID: {}): {} {} @ ${:.2}",
        player.name,
        player_sell_order_abc.id,
        player_sell_order_abc.quantity,
        player_sell_order_abc.security_ticker,
        player_sell_order_abc.price as f64 / 100.0
    );
    match stock_exchange.place_order(player_sell_order_abc.clone()) {
        Ok(trades) => {
            if trades.is_empty() {
                println!("  Order added to book.");
            }
            for trade in &trades {
                println!(
                    "  TRADE EXECUTED: {} {} @ ${:.2} (Buy Order: {}, Sell Order: {})",
                    trade.quantity,
                    trade.security_ticker,
                    trade.price as f64 / 100.0,
                    trade.matched_buy_order_id,
                    trade.matched_sell_order_id
                );
                if trade.matched_sell_order_id == player_sell_order_abc.id {
                    player.cash_balance += trade.quantity * trade.price;
                    player
                        .remove_shares(&trade.security_ticker, trade.quantity)
                        .unwrap();
                    println!(
                        "    Player's sell order (ID: {}) part of this trade. Cash/Portfolio updated.",
                        player_sell_order_abc.id
                    );
                }
            }
        }
        Err(e) => eprintln!("  Error placing order: {}", e),
    }

    // Player wants to buy 20 ABC at $76.00 (will try to match against player_sell_order_abc if any remains, or other market orders)
    current_sim_time += 1;
    let player_buy_order_abc_2 = Order::new(
        next_order_id_counter,
        OrderType::Buy,
        "ABC",
        20,
        7600,
        current_sim_time,
    );
    next_order_id_counter += 1;
    println!(
        "{} places BUY order (ID: {}): {} {} @ ${:.2}",
        player.name,
        player_buy_order_abc_2.id,
        player_buy_order_abc_2.quantity,
        player_buy_order_abc_2.security_ticker,
        player_buy_order_abc_2.price as f64 / 100.0
    );
    match stock_exchange.place_order(player_buy_order_abc_2.clone()) {
        Ok(trades) => {
            if trades.is_empty() {
                println!("  Order added to book.");
            }
            for trade in &trades {
                println!(
                    "  TRADE EXECUTED: {} {} @ ${:.2} (Buy Order: {}, Sell Order: {})",
                    trade.quantity,
                    trade.security_ticker,
                    trade.price as f64 / 100.0,
                    trade.matched_buy_order_id,
                    trade.matched_sell_order_id
                );
                if trade.matched_buy_order_id == player_buy_order_abc_2.id {
                    player.cash_balance -= trade.quantity * trade.price;
                    player.add_shares(&trade.security_ticker, trade.quantity, trade.price);
                    println!(
                        "    Player's buy order (ID: {}) part of this trade. Portfolio updated.",
                        player_buy_order_abc_2.id
                    );
                } else if trade.matched_sell_order_id == player_sell_order_abc.id {
                    // This case handles if the player's earlier sell order (player_sell_order_abc) is matched by someone else's buy.
                    // The logic for player selling is already in the block for player_sell_order_abc.
                    // This is more for if an external buy matches the player's sell.
                    // The current structure updates the player if *their* order is one of the matched_x_order_id.
                    println!(
                        "    Player's sell order (ID: {}) was matched by this buy.",
                        player_sell_order_abc.id
                    );
                }
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

    println!("\n--- Final Player Status ---");
    println!(
        "  {}: Cash ${:.2}",
        player.name,
        player.cash_balance as f64 / 100.0,
    );
    if player.portfolio.is_empty() {
        println!("    Portfolio is empty.");
    } else {
        println!("    Portfolio:");
        for (ticker, item) in &player.portfolio {
            println!(
                "      {}: {} shares @ avg buy price ${:.2}",
                ticker,
                item.quantity,
                item.average_buy_price as f64 / 100.0
            );
        }
    }

    let mut market_prices = std::collections::HashMap::new();
    for (ticker, security) in &stock_exchange.securities {
        market_prices.insert(ticker.clone(), security.current_price);
    }
    println!(
        "  {} Net Worth: ${:.2}",
        player.name,
        player.get_total_net_worth(&market_prices) as f64 / 100.0
    );
}
