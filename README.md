# Zero Sum Game - Stock Exchange Simulation

A command-line based (a user interface will be added later) stock exchange simulation written in Rust. This project models basic stock exchange functionalities including placing buy/sell orders, matching trades, and managing securities and trader portfolios.

## Features (Current/Planned)

* **Stock Exchange Core**: Simulates a stock exchange environment.
* **Securities Management**: Add and manage different stock securities with ticker symbols, names, industries, and initial prices.
* **Trader Simulation**: Create traders with initial cash balances and track their portfolios.
* **Order Matching**: Supports limit buy and sell orders with a basic matching engine.
* **Order Book**: Maintains buy and sell order books for each security.
* **Trade Execution & History**: Records executed trades.
* **Portfolio Tracking**: Calculates trader net worth based on cash and current market prices of owned shares.

*(Based on the provided Rust files, this seems to be a command-line application. If Bevy integration for a GUI is planned, you can add details about that here.)*

## Getting Started

### Prerequisites

* Rust programming language and Cargo package manager. (Install from "https://www.rust-lang.org/tools/install")

### Building

1.  Clone the repository (once it's on GitHub):
    ```bash
    git clone [https://github.com/SteveWinschel/zero-sum-game.git](https://github.com/SteveWinschel/zero-sum-game.git)
    cd zero-sum-game
    ```
2.  Build the project:
    Compilation (optimized):
    ```bash
    cargo build --release
    ```

### Running

Execute the compiled binary:
```bash
cargo run --release
