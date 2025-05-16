// src/main.rs

// Existing mods
mod exchange;
mod orders;
mod player;
mod securities;
mod securities_lib;
mod tui;
mod tui_commands;

use crate::exchange::StockExchange;
use crate::player::Player;
use crate::securities::get_initial_securities;
// Note: Order and OrderType might only be needed by command_handler now
// use crate::orders::{Order, OrderType}; // Potentially remove if not directly used in main

fn main() {
    tui::display_welcome_message();

    let mut next_order_id_counter: u64 = 1;
    let mut current_sim_time: u64 = 0;

    let mut stock_exchange = StockExchange::new();
    println!("\nStock Exchange created."); // Or move to a ui function

    let initial_securities = get_initial_securities();
    for sec in initial_securities {
        if let Err(e) = stock_exchange.add_security(sec) {
            eprintln!("Error adding security: {}", e); // Or ui::display_error
        }
    }

    tui::display_initial_securities(&stock_exchange);

    let mut player = Player::new("Player 1".to_string(), 1000000); // $10,000.00
    tui::display_player_status(&player);

    loop {
        tui::print_prompt();

        match tui::read_input() {
            Ok(input) => {
                current_sim_time += 1; // Advance time for each command attempt
                if !tui_commands::process_command(
                    &input,
                    &mut player,
                    &mut stock_exchange,
                    &mut next_order_id_counter,
                    current_sim_time,
                ) {
                    break; // Exit command was processed
                }
            }
            Err(e) => {
                tui::display_error(&format!("Failed to read input: {}. Please try again.", e));
            }
        }
    }
}
