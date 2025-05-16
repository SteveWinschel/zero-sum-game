# Zero Sum Game: A Rust-Based Security Exchange Simulation (Learning Project)

**Zero Sum Game** is a security exchange simulation built in Rust. This is my first significant project, undertaken primarily for learning and fun, aiming to explore software development concepts through the creation of a system that models core financial market dynamics. While it currently outputs to the console, the goal is to simulate functionalities like order matching, securities management, and portfolio tracking.

This project serves as a personal exploration into Rust programming, financial technology (FinTech) concepts, and simulation development.

## Key Features (Current & In-Progress)

* **Security Exchange Simulation Core**: Models the fundamental operations of a securities exchange.
* **Comprehensive Securities Management**: Allows for defining and managing various securities (ticker, name, industry, initial price).
* **Order Matching Engine**: Implements logic for limit buy and sell orders with a price-time priority matching system.
* **Detailed Order Books**: Maintains buy (bid) and sell (ask) order books for each security.
* **Trader Simulation & Portfolio Management**: Simulates a trader with a cash balance, tracking their portfolio value and net worth.
* **Trade Execution and History**: Records executed trades.
* **Modular Rust Codebase**: Developed with a focus on a clean and understandable structure in Rust.

## Technologies Used

* **Programming Language**: [Rust](https://www.rust-lang.org/)
* **Key Crates/Libraries**:
    * Bevy (engine)
    * rand (for RNG)

## Getting Started

Follow these instructions to get a copy of the project up and running on your local machine for exploration and development.

### Prerequisites

* **Rust Programming Language**: Ensure you have Rust and Cargo (the Rust package manager) installed. You can download them from [rust-lang.org](https://www.rust-lang.org/tools/install).

### Installation & Building

1.  **Clone the Repository**:
    ```bash
    git clone [https://github.com/SteveWinschel/zero-sum-game.git](https://github.com/SteveWinschel/zero-sum-game.git)
    cd zero-sum-game
    ```

2.  **Build the Project**:
    For a development build:
    ```bash
    cargo build
    ```
    For an optimized release build:
    ```bash
    cargo build --release
    ```

### Running the Simulation

Execute the compiled binary from the project root:
```bash
# If you built with --release
./target/release/zero-sum-game

# If you built for development
./target/debug/zero-sum-game

# Alternatively, you can run directly using Cargo:
cargo run
# or for release
cargo run --release
