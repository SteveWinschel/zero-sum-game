use std::borrow::Cow;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Industry {
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

impl fmt::Display for Industry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Trend {
    Normal,
    Rising,
    Correcting,
    Bubble,
    ShortSqueeze,
    Banned,
    PanicSelling,
    PanicBuying,
    InsidersBuying,
    InsidersSelling,
}

// For displaying trend names
impl fmt::Display for Trend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Security {
    pub ticker_symbol: Cow<'static, str>,
    pub name: Cow<'static, str>,
    pub industry: Industry,
    pub current_price: u64,
    pub previous_close_price: u64,
    pub open_price: u64,
    pub day_high: u64,
    pub day_low: u64,
    pub volume: u64,
    pub market_cap: u64,
    pub outstanding_shares: u64,
    pub trend: Trend, // Replaced tradable: bool with the "Trend" enum
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
            previous_close_price: initial_price,
            open_price: initial_price,
            day_high: initial_price,
            day_low: initial_price,
            volume: 0,
            market_cap: initial_price * outstanding_shares,
            outstanding_shares,
            trend: Trend::Normal, // To initialize trend, e.g., to Normal by default
        }
    }

    pub fn update_price(&mut self, new_price: u64) {
        self.current_price = new_price;
        if new_price > self.day_high {
            self.day_high = new_price;
        }
        if new_price < self.day_low {
            self.day_low = new_price;
        }
        self.market_cap = self.current_price * self.outstanding_shares;
    }

    pub fn record_trade(&mut self, traded_price: u64, quantity: u64) {
        self.current_price = traded_price;
        self.volume += quantity;
        if traded_price > self.day_high {
            self.day_high = traded_price;
        }
        if traded_price < self.day_low {
            self.day_low = traded_price;
        }
        self.market_cap = self.current_price * self.outstanding_shares;
    }

    pub fn close_day(&mut self) {
        self.previous_close_price = self.current_price;
    }

    pub fn open_day(&mut self, open_price: u64) {
        self.open_price = open_price;
        self.current_price = open_price;
        self.day_high = open_price;
        self.day_low = open_price;
        self.volume = 0;
    }
}

impl fmt::Display for Security {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Ticker: {}", self.ticker_symbol)?;
        writeln!(f, "  Name: {}", self.name)?;
        writeln!(f, "  Industry: {}", self.industry)?;
        writeln!(f, "  Price: ${:.2}", self.current_price as f64 / 100.0)?;
        writeln!(f, "  Volume: {}", self.volume)?;
        writeln!(f, "  Market Cap: ${:.2}", self.market_cap as f64 / 100.0)?;
        writeln!(f, "  Trend: {}", self.trend) // Updated to display trend
    }
}
