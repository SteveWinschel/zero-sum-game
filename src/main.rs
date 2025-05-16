mod exchange;
mod orders;
mod player;
mod securities;
mod securities_lib;
use crate::orders::{Order, OrderType, Trade};
use std::io::{self, Write}; // Add for input/output

fn main() {
    println!("--- Zero Sum Game Stock Exchange Simulation ---");

    let mut next_order_id_counter: u64 = 1;
    let mut current_sim_time: u64 = 0;

    let mut stock_exchange = StockExchange::new();
    println!("\nStock Exchange created.");

    let initial_securities = get_initial_securities();
    for sec in initial_securities {
        // println!("Adding security: {}", sec.ticker_symbol); // Potentially less verbose for interactive
        if let Err(e) = stock_exchange.add_security(sec) {
            eprintln!("Error adding security: {}", e);
        }
    }

    // Display initial securities once
    println!("\n--- Available Securities ---");
    for ticker in stock_exchange.securities.keys() {
        if let Some(sec_detail) = stock_exchange.get_security(ticker) {
            println!(
                "  {}: {} - Price: ${:.2}",
                sec_detail.ticker_symbol,
                sec_detail.name,
                sec_detail.current_price as f64 / 100.0
            );
        }
    }

    let mut player = Player::new("Player 1".to_string(), 1000000); // $10,000.00
    println!(
        "\nWelcome, {}! Your cash balance: ${:.2}",
        player.name,
        player.cash_balance as f64 / 100.0
    );

    loop {
        print!("\n> "); // Prompt
        io::stdout().flush().unwrap(); // Ensure '>' is printed before input

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input. Please try again.");
            continue;
        }

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let command = parts[0].to_lowercase();
        current_sim_time += 1; // Increment simulation time per command or per order

        match command.as_str() {
            "buy" => {
                // Expected: buy <TICKER> <QUANTITY> <PRICE_PER_SHARE>
                // Example: buy XYZ 10 150.00
                if parts.len() == 4 {
                    let ticker = parts[1].to_uppercase();
                    let quantity: Result<u64, _> = parts[2].parse();
                    let price_str = parts[3];
                    // Convert price (e.g., "150.00") to u64 cents
                    let price: Result<u64, _> = price_str.replace('.', "").parse();

                    match (quantity, price) {
                        (Ok(qty), Ok(prc)) => {
                            if !stock_exchange.securities.contains_key(ticker.as_str()) {
                                println!("Error: Security {} not found.", ticker);
                                continue;
                            }
                            let total_cost = qty * prc;
                            if player.cash_balance < total_cost {
                                println!(
                                    "Error: Insufficient funds. Required: ${:.2}, Available: ${:.2}",
                                    total_cost as f64 / 100.0,
                                    player.cash_balance as f64 / 100.0
                                );
                                continue;
                            }

                            let order = Order::new(
                                next_order_id_counter,
                                OrderType::Buy,
                                Box::leak(ticker.into_boxed_str()), // Convert String to &'static str
                                qty,
                                prc,
                                current_sim_time,
                            );
                            next_order_id_counter += 1;

                            println!(
                                "Placing BUY order (ID: {}): {} {} @ ${:.2}",
                                order.id,
                                order.quantity,
                                order.security_ticker,
                                order.price as f64 / 100.0
                            );

                            match stock_exchange.place_order(order.clone()) {
                                Ok(trades) => {
                                    if trades.is_empty() {
                                        println!(
                                            "  Order for {} added to book.",
                                            order.security_ticker
                                        );
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
                                        // If this player's buy order was part of the trade
                                        if trade.matched_buy_order_id == order.id {
                                            player.cash_balance -= trade.quantity * trade.price;
                                            player.add_shares(
                                                &trade.security_ticker, // Already &'static str from order
                                                trade.quantity,
                                                trade.price,
                                            );
                                            println!(
                                                "    Your portfolio updated. New cash balance: ${:.2}",
                                                player.cash_balance as f64 / 100.0
                                            );
                                        }
                                    }
                                }
                                Err(e) => eprintln!("  Error placing order: {}", e),
                            }
                        }
                        _ => println!(
                            "Invalid quantity or price format. Usage: buy <TICKER> <QUANTITY> <PRICE_IN_DOLLARS.CENTS> (e.g., buy XYZ 10 150.00)"
                        ),
                    }
                } else {
                    println!(
                        "Usage: buy <TICKER> <QUANTITY> <PRICE_IN_DOLLARS.CENTS> (e.g., buy XYZ 10 150.00)"
                    );
                }
            }
            "sell" => {
                // Expected: sell <TICKER> <QUANTITY> <PRICE_PER_SHARE>
                // Example: sell ABC 5 75.50
                if parts.len() == 4 {
                    let ticker = parts[1].to_uppercase();
                    let quantity_to_sell: Result<u64, _> = parts[2].parse();
                    let price_str = parts[3];
                    let price: Result<u64, _> = price_str.replace('.', "").parse();

                    match (quantity_to_sell, price) {
                        (Ok(qty_sell), Ok(prc_sell)) => {
                            // Check if player has enough shares
                            let owned_shares = player
                                .portfolio
                                .get(ticker.as_str())
                                .map_or(0, |item| item.quantity);
                            if owned_shares < qty_sell {
                                println!(
                                    "Error: Not enough {} shares to sell. You own: {}",
                                    ticker, owned_shares
                                );
                                continue;
                            }
                            if !stock_exchange.securities.contains_key(ticker.as_str()) {
                                println!("Error: Security {} not found.", ticker);
                                continue;
                            }

                            let order = Order::new(
                                next_order_id_counter,
                                OrderType::Sell,
                                Box::leak(ticker.into_boxed_str()), // Convert String to &'static str
                                qty_sell,
                                prc_sell,
                                current_sim_time,
                            );
                            next_order_id_counter += 1;

                            println!(
                                "Placing SELL order (ID: {}): {} {} @ ${:.2}",
                                order.id,
                                order.quantity,
                                order.security_ticker,
                                order.price as f64 / 100.0
                            );

                            match stock_exchange.place_order(order.clone()) {
                                Ok(trades) => {
                                    if trades.is_empty() {
                                        println!(
                                            "  Order for {} added to book.",
                                            order.security_ticker
                                        );
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
                                        // If this player's sell order was part of the trade
                                        if trade.matched_sell_order_id == order.id {
                                            player.cash_balance += trade.quantity * trade.price;
                                            match player.remove_shares(
                                                &trade.security_ticker,
                                                trade.quantity,
                                            ) {
                                                Ok(()) => println!(
                                                    "    Your portfolio updated. New cash balance: ${:.2}",
                                                    player.cash_balance as f64 / 100.0
                                                ),
                                                Err(e) => eprintln!(
                                                    "    Error updating portfolio after sell: {}",
                                                    e
                                                ), // Should not happen if pre-check was done
                                            }
                                        }
                                    }
                                }
                                Err(e) => eprintln!("  Error placing order: {}", e),
                            }
                        }
                        _ => println!(
                            "Invalid quantity or price format. Usage: sell <TICKER> <QUANTITY> <PRICE_IN_DOLLARS.CENTS> (e.g., sell XYZ 10 150.00)"
                        ),
                    }
                } else {
                    println!(
                        "Usage: sell <TICKER> <QUANTITY> <PRICE_IN_DOLLARS.CENTS> (e.g., sell XYZ 10 150.00)"
                    );
                }
            }
            "portfolio" | "pf" => {
                println!("\n--- Your Portfolio ---");
                println!("Cash Balance: ${:.2}", player.cash_balance as f64 / 100.0);
                if player.portfolio.is_empty() {
                    println!("  No securities held.");
                } else {
                    for (ticker, item) in &player.portfolio {
                        let current_market_price = stock_exchange
                            .securities
                            .get(ticker.as_ref()) // Cow<&'static str> to &str
                            .map_or(0, |sec| sec.current_price);
                        let market_value = item.quantity * current_market_price;
                        println!(
                            "  {}: {} shares @ avg buy price ${:.2} (Current Market Value: ${:.2})",
                            ticker,
                            item.quantity,
                            item.average_buy_price as f64 / 100.0,
                            market_value as f64 / 100.0
                        );
                    }
                }
                let mut market_prices_map = std::collections::HashMap::new();
                for (ticker_cow, security) in &stock_exchange.securities {
                    market_prices_map.insert(ticker_cow.clone(), security.current_price);
                }
                println!(
                    "Total Net Worth: ${:.2}",
                    player.get_total_net_worth(&market_prices_map) as f64 / 100.0
                );
            }
            "prices" | "quote" => {
                // Expected: prices or prices <TICKER>
                println!("\n--- Current Market Prices ---");
                if parts.len() > 1 {
                    let ticker_to_quote = parts[1].to_uppercase();
                    if let Some(sec) = stock_exchange.get_security(&ticker_to_quote) {
                        println!("{}", sec); // Uses the Display impl for Security
                    } else {
                        println!("Security {} not found.", ticker_to_quote);
                    }
                } else {
                    for ticker in stock_exchange.securities.keys() {
                        if let Some(sec_detail) = stock_exchange.get_security(ticker) {
                            println!(
                                "  {}: {} - Price: ${:.2}, Volume: {}",
                                sec_detail.ticker_symbol,
                                sec_detail.name,
                                sec_detail.current_price as f64 / 100.0,
                                sec_detail.volume
                            );
                        }
                    }
                }
            }
            "orderbook" | "ob" => {
                // Expected: orderbook <TICKER>
                if parts.len() == 2 {
                    let ticker_str = parts[1].to_uppercase();
                    if let Some(book_display) = stock_exchange.get_order_book_display(&ticker_str) {
                        println!("\n{}", book_display);
                    } else {
                        println!(
                            "No order book found for ticker {} (it might not be a listed security).",
                            ticker_str
                        );
                    }
                } else {
                    println!("Usage: orderbook <TICKER>");
                }
            }
            "trades" => {
                println!("\n{}", stock_exchange.get_trade_history_display());
            }
            "help" => {
                println!("\nAvailable commands:");
                println!(
                    "  buy <TICKER> <QUANTITY> <PRICE>   - Place a buy order (e.g., buy XYZ 10 150.00)"
                );
                println!(
                    "  sell <TICKER> <QUANTITY> <PRICE>  - Place a sell order (e.g., sell ABC 5 75.50)"
                );
                println!(
                    "  portfolio (or pf)                 - Show your current cash and holdings."
                );
                println!(
                    "  prices (or quote) [TICKER]      - Show current market prices for all or a specific security."
                );
                println!(
                    "  orderbook (or ob) <TICKER>      - Show the order book for a specific security."
                );
                println!("  trades                            - Show the history of all trades.");
                println!("  help                              - Show this help message.");
                println!("  exit                              - Exit the game.");
            }
            "exit" => {
                println!("Exiting simulation. Thank you for playing!");
                break;
            }
            _ => {
                println!(
                    "Unknown command: '{}'. Type 'help' for a list of commands.",
                    command
                );
            }
        }
    }
}
