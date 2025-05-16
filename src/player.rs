use std::borrow::Cow;
use std::collections::HashMap;

use crate::securities_lib::Security; // Assuming Security is needed here, adjust if not

#[derive(Debug)]
pub struct PortfolioItem {
    pub security_ticker: Cow<'static, str>,
    pub quantity: u64,
    pub average_buy_price: u64, // Optional: to track cost basis
}

#[derive(Debug)]
pub struct Player {
    pub name: &'static &str, // For display purposes, can be kept
    pub cash_balance: u64,   // In cents
    pub portfolio: HashMap<Cow<'static, str>, PortfolioItem>, // Ticker symbol -> PortfolioItem
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

    pub fn add_shares(
        &mut self,
        security_ticker: &'static str,
        quantity: u64,
        purchase_price: u64,
    ) {
        let ticker_cow = Cow::Borrowed(security_ticker);
        let item = self
            .portfolio
            .entry(ticker_cow.clone())
            .or_insert_with(|| PortfolioItem {
                security_ticker: ticker_cow,
                quantity: 0,
                average_buy_price: 0, // Will be updated
            });

        // Update average buy price (weighted average)
        let new_total_value =
            (item.quantity * item.average_buy_price) + (quantity * purchase_price);
        item.quantity += quantity;
        if item.quantity > 0 {
            // Avoid division by zero
            item.average_buy_price = new_total_value / item.quantity;
        } else {
            item.average_buy_price = 0; // Should not happen if quantity > 0
        }
    }

    pub fn remove_shares(
        &mut self,
        security_ticker: &'static str,
        quantity: u64,
    ) -> Result<(), &'static str> {
        let ticker_cow = Cow::Borrowed(security_ticker);
        if let Some(item) = self.portfolio.get_mut(&ticker_cow) {
            if item.quantity >= quantity {
                item.quantity -= quantity;
                // If quantity becomes 0, you might want to remove the item from the portfolio
                if item.quantity == 0 {
                    self.portfolio.remove(&ticker_cow);
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
            if let Some(current_price) = market_prices.get(ticker) {
                total_value += item.quantity * current_price;
            }
            // else: security in portfolio but no current market price? Handle as error or skip.
        }
        total_value
    }

    pub fn get_total_net_worth(&self, market_prices: &HashMap<Cow<'static, str>, u64>) -> u64 {
        self.cash_balance + self.get_portfolio_value(market_prices)
    }
}
