// I will add a graphical user interface one day but it is not a top priority at the moment

use crate::exchange::StockExchange;
use crate::player::Player;
use std::collections::HashMap;
use std::io::{self, Write};

pub fn display_welcome_message() {
    println!("--- Zero Sum Game Stock Exchange Simulation ---");
}

pub fn display_initial_securities(stock_exchange: &StockExchange) {
    println!("\n--- Available Securities ---");
    for ticker_cow in stock_exchange.securities.keys() {
        if let Some(sec_detail) = stock_exchange.get_security(ticker_cow.as_ref()) {
            println!(
                "  {}: {} - Price: ${:.2}",
                sec_detail.ticker_symbol,
                sec_detail.name,
                sec_detail.current_price as f64 / 100.0
            );
        }
    }
}

pub fn display_player_status(player: &Player) {
    println!(
        "\nWelcome, {}! Your cash balance: ${:.2}",
        player.name,
        player.cash_balance as f64 / 100.0
    );
}

pub fn print_prompt() {
    print!("\n> ");
    io::stdout().flush().unwrap();
}

pub fn read_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

pub fn display_portfolio(player: &Player, stock_exchange: &StockExchange) {
    println!("\n--- Your Portfolio ---");
    println!("Cash Balance: ${:.2}", player.cash_balance as f64 / 100.0);
    if player.portfolio.is_empty() {
        println!("  No securities held.");
    } else {
        for (ticker_cow, item) in &player.portfolio {
            let current_market_price = stock_exchange
                .securities
                .get(ticker_cow.as_ref())
                .map_or(0, |sec| sec.current_price);
            let market_value = item.quantity * current_market_price;
            println!(
                "  {}: {} shares @ avg buy price ${:.2} (Current Market Value: ${:.2})",
                ticker_cow,
                item.quantity,
                item.average_buy_price as f64 / 100.0,
                market_value as f64 / 100.0
            );
        }
    }
    let mut market_prices_map = HashMap::new();
    for (ticker_cow, security) in &stock_exchange.securities {
        market_prices_map.insert(ticker_cow.clone(), security.current_price);
    }
    println!(
        "Total Net Worth: ${:.2}",
        player.get_total_net_worth(&market_prices_map) as f64 / 100.0
    );
}

pub fn display_market_prices(stock_exchange: &StockExchange, specific_ticker: Option<&str>) {
    println!("\n--- Current Market Prices ---");
    if let Some(ticker_to_quote) = specific_ticker {
        if let Some(sec) = stock_exchange.get_security(&ticker_to_quote.to_uppercase()) {
            println!("{}", sec); // Assumes Security has a Display impl
        } else {
            println!("Security {} not found.", ticker_to_quote);
        }
    } else {
        for ticker_cow in stock_exchange.securities.keys() {
            if let Some(sec_detail) = stock_exchange.get_security(ticker_cow.as_ref()) {
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

pub fn display_order_book(stock_exchange: &StockExchange, ticker: &str) {
    if let Some(book_display) = stock_exchange.get_order_book_display(&ticker.to_uppercase()) {
        println!("\n{}", book_display);
    } else {
        println!(
            "No order book found for ticker {} (it might not be a listed security).",
            ticker
        );
    }
}

pub fn display_trade_history(stock_exchange: &StockExchange) {
    println!("\n{}", stock_exchange.get_trade_history_display());
}

pub fn display_help() {
    println!("\nAvailable commands:");
    println!("  buy <TICKER> <QUANTITY>           - Place a market buy order (e.g., buy XYZ 10)");
    println!("  sell <TICKER> <QUANTITY>          - Place a market sell order (e.g., sell ABC 5)");
    println!("  portfolio (or pf)                 - Show your current cash and holdings.");
    println!("  prices (or quote) [TICKER]      - Show current market prices.");
    println!("  orderbook (or ob) <TICKER>      - Show the order book.");
    println!("  trades                            - Show the history of all trades.");
    println!("  help                              - Show this help message.");
    println!("  exit                              - Exit the game.");
}

pub fn display_exit_message() {
    println!("Exiting simulation. Thank you for playing!");
}

pub fn display_error(message: &str) {
    eprintln!("Error: {}", message);
}

pub fn display_usage(command: &str, usage: &str) {
    println!("Usage: {} {}", command, usage);
}
