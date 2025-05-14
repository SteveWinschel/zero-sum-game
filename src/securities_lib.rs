use std::borrow::Cow;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)] // Added derive for common traits
pub enum Industry {
    // Made public
    Technology,
    Finance,
    Healthcare,
    Retail,
    Manufacturing,
    Energy,
    Utilities,
    RealEstate,
    Media,
    ConsumerGoods,
    Transportation,
    Agriculture,
    Education,
    Government,
    Blockchain,
    Other,
}

impl Industry {
    // Consider making this public if needed outside this module
    pub fn new(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "technology" => Industry::Technology,
            "finance" => Industry::Finance,
            "healthcare" => Industry::Healthcare,
            "retail" => Industry::Retail,
            "manufacturing" => Industry::Manufacturing,
            "energy" => Industry::Energy,
            "utilities" => Industry::Utilities,
            "realestate" | "real estate" => Industry::RealEstate,
            "media" => Industry::Media,
            "consumergoods" | "consumer goods" => Industry::ConsumerGoods,
            "transportation" => Industry::Transportation,
            "agriculture" => Industry::Agriculture,
            "education" => Industry::Education,
            "government" => Industry::Government,
            "blockchain" => Industry::Blockchain,
            _ => Industry::Other,
        }
    }
}

// For displaying industry names
impl fmt::Display for Industry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)] // Added derive for common traits
pub struct Security {
    // Made public
    pub ticker_symbol: Cow<'static, str>, // Unique identifier for the stock (e.g., "AAPL")
    pub name: Cow<'static, str>,
    pub industry: Industry,
    pub current_price: u64,        // Price in cents for fixed point arithmetic
    pub previous_close_price: u64, // Price at the end of the previous trading day
    pub open_price: u64,           // Price at the start of the current trading day
    pub day_high: u64,             // Highest price during the current trading day
    pub day_low: u64,              // Lowest price during the current trading day
    pub volume: u64,               // Number of shares traded during the current day
    pub market_cap: u64,           // Total market value (current_price * outstanding_shares)
    pub outstanding_shares: u64,   // Total number of shares issued by the company
    pub tradable: bool,            // Mainly for restricting trade (e.g., on weekends, or if halted)
}

impl Security {
    pub fn new(
        ticker_symbol: &'static str,
        name: &'static str,
        industry: Industry,
        initial_price: u64,
        outstanding_shares: u64,
    ) -> Self {
        Security {
            ticker_symbol: Cow::Borrowed(ticker_symbol),
            name: Cow::Borrowed(name),
            industry,
            current_price: initial_price,
            previous_close_price: initial_price, // Assume previous close is the initial price for simplicity
            open_price: initial_price,           // Assume open is the initial price
            day_high: initial_price,
            day_low: initial_price,
            volume: 0,
            market_cap: initial_price * outstanding_shares,
            outstanding_shares,
            tradable: true, // Tradable by default
        }
    }

    // Example method to update price, could be more complex
    pub fn update_price(&mut self, new_price: u64) {
        self.current_price = new_price;
        if new_price > self.day_high {
            self.day_high = new_price;
        }
        if new_price < self.day_low {
            self.day_low = new_price;
        }
        // Market cap should also be updated
        self.market_cap = self.current_price * self.outstanding_shares;
    }

    // Method to handle a trade occurring
    pub fn record_trade(&mut self, traded_price: u64, quantity: u64) {
        self.current_price = traded_price; // Last traded price becomes current price
        self.volume += quantity;
        if traded_price > self.day_high {
            self.day_high = traded_price;
        }
        if traded_price < self.day_low {
            self.day_low = traded_price;
        }
        self.market_cap = self.current_price * self.outstanding_shares;
    }

    // Potentially a method to be called at the end of a trading day
    pub fn close_day(&mut self) {
        self.previous_close_price = self.current_price;
        // Reset day's high, low, volume for the next day
        // Open price for next day would be set at market open
    }

    // Potentially a method to be called at the start of a trading day
    pub fn open_day(&mut self, open_price: u64) {
        self.open_price = open_price;
        self.current_price = open_price;
        self.day_high = open_price;
        self.day_low = open_price;
        self.volume = 0;
    }
}

// For displaying security information
impl fmt::Display for Security {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Ticker: {}", self.ticker_symbol)?;
        writeln!(f, "  Name: {}", self.name)?;
        writeln!(f, "  Industry: {}", self.industry)?;
        writeln!(f, "  Price: ${:.2}", self.current_price as f64 / 100.0)?;
        writeln!(f, "  Volume: {}", self.volume)?;
        writeln!(f, "  Market Cap: ${:.2}", self.market_cap as f64 / 100.0)?;
        writeln!(f, "  Tradable: {}", self.tradable)
    }
}
