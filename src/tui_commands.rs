use crate::exchange::StockExchange;
use crate::orders::{Order, OrderType};
use crate::player::Player;
use crate::tui;

// Helper function to handle buy commands
fn handle_buy_command(
    parts: &[&str],
    player: &mut Player,
    stock_exchange: &mut StockExchange,
    next_order_id_counter: &mut u64,
    current_sim_time: u64,
) {
    if parts.len() == 3 {
        let ticker_string = parts[1].to_uppercase();
        match parts[2].parse::<u64>() {
            Ok(qty) => {
                if qty == 0 {
                    tui::display_error("Quantity must be greater than 0.");
                    return;
                }
                let current_market_price = match stock_exchange.get_security(&ticker_string) {
                    Some(sec) => sec.current_price,
                    None => {
                        tui::display_error(&format!("Security {} not found.", ticker_string));
                        return;
                    }
                };
                if current_market_price == 0 {
                    tui::display_error(&format!(
                        "Security {} has a market price of $0.00, cannot buy.",
                        ticker_string
                    ));
                    return;
                }

                let total_cost = qty * current_market_price;
                if player.cash_balance < total_cost {
                    tui::display_error(&format!(
                        "Insufficient funds. Required: ${:.2} ({} x ${:.2}), Available: ${:.2}",
                        total_cost as f64 / 100.0,
                        qty,
                        current_market_price as f64 / 100.0,
                        player.cash_balance as f64 / 100.0
                    ));
                    return;
                }

                let order = Order::new(
                    *next_order_id_counter,
                    OrderType::Buy,
                    Box::leak(ticker_string.into_boxed_str()),
                    qty,
                    current_market_price,
                    current_sim_time,
                );
                *next_order_id_counter += 1;

                println!(
                    "Placing MARKET BUY order (ID: {}): {} {} @ market price (approx. ${:.2})",
                    order.id,
                    order.quantity,
                    order.security_ticker,
                    order.price as f64 / 100.0
                );

                match stock_exchange.place_order(order.clone()) {
                    Ok(trades) => {
                        if trades.is_empty() {
                            println!(
                                "  Order for {} could not be filled at current market price.",
                                order.security_ticker
                            );
                        }
                        for trade_item in trades {
                            println!(
                                "  TRADE EXECUTED: {} {} @ ${:.2} (Order ID: {})",
                                trade_item.quantity,
                                trade_item.security_ticker,
                                trade_item.price as f64 / 100.0,
                                if order.order_type == OrderType::Buy {
                                    trade_item.matched_buy_order_id
                                } else {
                                    trade_item.matched_sell_order_id
                                }
                            );
                            player.cash_balance -= trade_item.quantity * trade_item.price;
                            player.add_shares(
                                trade_item.security_ticker.clone(),
                                trade_item.quantity,
                                trade_item.price,
                            );
                            println!(
                                "    Your portfolio updated. New cash balance: ${:.2}",
                                player.cash_balance as f64 / 100.0
                            );
                        }
                    }
                    Err(e) => eprintln!("  Error placing market buy order: {}", e),
                }
            }
            _ => tui::display_usage("buy", "<TICKER> <QUANTITY> (e.g., buy XYZ 10)"),
        }
    } else {
        tui::display_usage("buy", "<TICKER> <QUANTITY> (e.g., buy XYZ 10)");
    }
}

// Helper function to handle sell commands
fn handle_sell_command(
    parts: &[&str],
    player: &mut Player,
    stock_exchange: &mut StockExchange,
    next_order_id_counter: &mut u64,
    current_sim_time: u64,
) {
    if parts.len() == 3 {
        let ticker_string = parts[1].to_uppercase();
        match parts[2].parse::<u64>() {
            Ok(qty_sell) => {
                if qty_sell == 0 {
                    tui::display_error("Quantity must be greater than 0.");
                    return;
                }
                let owned_shares = player
                    .portfolio
                    .get(ticker_string.as_str())
                    .map_or(0, |item| item.quantity);

                if owned_shares < qty_sell {
                    tui::display_error(&format!(
                        "Not enough {} shares to sell. You own: {}, attempting to sell: {}",
                        ticker_string, owned_shares, qty_sell
                    ));
                    return;
                }

                let current_market_price = match stock_exchange.get_security(&ticker_string) {
                    Some(sec) => sec.current_price,
                    None => {
                        tui::display_error(&format!("Security {} not found.", ticker_string));
                        return;
                    }
                };
                if current_market_price == 0 {
                    tui::display_error(&format!(
                        "Security {} has a market price of $0.00, cannot sell.",
                        ticker_string
                    ));
                    return;
                }

                let order = Order::new(
                    *next_order_id_counter,
                    OrderType::Sell,
                    Box::leak(ticker_string.into_boxed_str()),
                    qty_sell,
                    current_market_price,
                    current_sim_time,
                );
                *next_order_id_counter += 1;

                println!(
                    "Placing MARKET SELL order (ID: {}): {} {} @ market price (approx. ${:.2})",
                    order.id,
                    order.quantity,
                    order.security_ticker,
                    order.price as f64 / 100.0
                );

                match stock_exchange.place_order(order.clone()) {
                    Ok(trades) => {
                        if trades.is_empty() {
                            println!(
                                "  Order for {} could not be filled at current market price.",
                                order.security_ticker
                            );
                        }
                        for trade_item in trades {
                            println!(
                                "  TRADE EXECUTED: {} {} @ ${:.2} (Order ID: {})",
                                trade_item.quantity,
                                trade_item.security_ticker,
                                trade_item.price as f64 / 100.0,
                                if order.order_type == OrderType::Buy {
                                    trade_item.matched_buy_order_id
                                } else {
                                    trade_item.matched_sell_order_id
                                }
                            );
                            player.cash_balance += trade_item.quantity * trade_item.price;
                            match player.remove_shares(
                                trade_item.security_ticker.clone(),
                                trade_item.quantity,
                            ) {
                                Ok(()) => println!(
                                    "    Your portfolio updated. New cash balance: ${:.2}",
                                    player.cash_balance as f64 / 100.0
                                ),
                                Err(e) => {
                                    eprintln!("    Error updating portfolio after sell: {}", e)
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("  Error placing market sell order: {}", e),
                }
            }
            _ => tui::display_usage("sell", "<TICKER> <QUANTITY> (e.g., sell XYZ 10)"),
        }
    } else {
        tui::display_usage("sell", "<TICKER> <QUANTITY> (e.g., sell XYZ 10)");
    }
}

// Main function to process commands
// Returns true if the simulation should continue, false if "exit" command was given.
pub fn process_command(
    input: &str,
    player: &mut Player,
    stock_exchange: &mut StockExchange,
    next_order_id_counter: &mut u64,
    current_sim_time: u64, // Kept as u64 since it's advanced in main loop
) -> bool {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() {
        return true; // Continue on empty input
    }

    let command = parts[0].to_lowercase();

    match command.as_str() {
        "buy" => handle_buy_command(
            &parts,
            player,
            stock_exchange,
            next_order_id_counter,
            current_sim_time,
        ),
        "sell" => handle_sell_command(
            &parts,
            player,
            stock_exchange,
            next_order_id_counter,
            current_sim_time,
        ),
        "portfolio" | "pf" => tui::display_portfolio(player, stock_exchange),
        "prices" | "quote" => {
            let ticker_arg = if parts.len() > 1 {
                Some(parts[1])
            } else {
                None
            };
            tui::display_market_prices(stock_exchange, ticker_arg);
        }
        "orderbook" | "ob" => {
            if parts.len() == 2 {
                tui::display_order_book(stock_exchange, parts[1]);
            } else {
                tui::display_usage("orderbook", "<TICKER>");
            }
        }
        "trades" => tui::display_trade_history(stock_exchange),
        "help" => tui::display_help(),
        "exit" => {
            tui::display_exit_message();
            return false; // Signal to exit loop
        }
        _ => {
            println!(
                "Unknown command: '{}'. Type 'help' for a list of commands.",
                command
            );
        }
    }
    true // Continue simulation
}
