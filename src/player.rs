use std::borrow::Cow;
use std::collections::HashMap;

// Removed unused import: crate::securities_lib::Security;

#[derive(Debug)]
pub struct PortfolioItem {
    pub security_ticker: Cow<'static, str>,
    pub quantity: u64,
    pub average_buy_price: u64, // Optional: to track cost basis
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub cash_balance: u64,
    pub portfolio: HashMap<Cow<'static, str>, PortfolioItem>,
}

impl Player {
    pub fn new(name: String, initial_cash: u64) -> Self {
        Player {
            name,
            cash_balance: initial_cash,
            portfolio: HashMap::new(),
        }
    }

    pub fn deposit_cash(&mut self, amount: u64) {
        self.cash_balance += amount;
    }

    pub fn withdraw_cash(&mut self, amount: u64) -> Result<(), &'static str> {
        if self.cash_balance >= amount {
            self.cash_balance -= amount;
            Ok(())
        } else {
            Err("Insufficient cash balance.")
        }
    }

    // Modified signature and implementation to accept Cow<'static, str>
    pub fn add_shares(
        &mut self,
        security_ticker: Cow<'static, str>, // Changed from &'static str
        quantity: u64,
        purchase_price: u64,
    ) {
        let item = self
            .portfolio
            .entry(security_ticker.clone()) // Clone Cow for key for entry API
            .or_insert_with(|| PortfolioItem {
                security_ticker, // Original Cow is moved into the PortfolioItem if new
                quantity: 0,
                average_buy_price: 0,
            });

        // Update average buy price (weighted average)
        let new_total_value =
            (item.quantity * item.average_buy_price) + (quantity * purchase_price);
        item.quantity += quantity;
        if item.quantity > 0 {
            // Avoid division by zero
            item.average_buy_price = new_total_value / item.quantity;
        } else {
            item.average_buy_price = 0; // Should not happen if quantity to add is > 0
        }
    }

    // Modified signature and implementation to accept Cow<'static, str>
    pub fn remove_shares(
        &mut self,
        security_ticker: Cow<'static, str>, // Changed from &'static str
        quantity: u64,
    ) -> Result<(), &'static str> {
        // .get_mut() and .remove() take &Q where K: Borrow<Q>.
        // We can pass &security_ticker (borrowing the Cow passed by value).
        if let Some(item) = self.portfolio.get_mut(&security_ticker) {
            if item.quantity >= quantity {
                item.quantity -= quantity;
                if item.quantity == 0 {
                    self.portfolio.remove(&security_ticker);
                }
                Ok(())
            } else {
                Err("Not enough shares to sell.")
            }
        } else {
            Err("Security not found in portfolio.")
        }
    }

    pub fn get_portfolio_value(&self, market_prices: &HashMap<Cow<'static, str>, u64>) -> u64 {
        let mut total_value = 0;
        for (ticker, item) in &self.portfolio {
            // ticker is &Cow<'static, str> from iterating the portfolio
            if let Some(current_price) = market_prices.get(ticker) {
                // HashMap::get can take &Cow
                total_value += item.quantity * current_price;
            }
        }
        total_value
    }

    pub fn get_total_net_worth(&self, market_prices: &HashMap<Cow<'static, str>, u64>) -> u64 {
        self.cash_balance + self.get_portfolio_value(market_prices)
    }
}
