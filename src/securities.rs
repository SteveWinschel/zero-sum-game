// Now 'crate::securities_lib' refers to the structs and enums we defined.
use crate::securities_lib::{Industry, Security};
use std::borrow::Cow;

pub fn get_initial_securities() -> Vec<Security> {
    vec![
        Security::new(
            "XYZ",
            "XYZ Corp",
            Industry::Technology,
            15000, // $150.00
            1_000_000,
        ),
        Security::new(
            "ABC",
            "ABC Materials",
            Industry::Manufacturing,
            7550, // $75.50
            500_000,
        ),
        Security::new(
            "FIN",
            "Finance Hub",
            Industry::Finance,
            25000, // $250.00
            750_000,
        ),
        Security {
            ticker_symbol: Cow::Borrowed("GAME"),
            name: Cow::Borrowed("Zero Sum Game Inc."),
            industry: Industry::Media,
            current_price: 1000, // $10.00
            previous_close_price: 1000,
            open_price: 1000,
            day_high: 1000,
            day_low: 1000,
            volume: 0,
            market_cap: 1000 * 200000,
            outstanding_shares: 200000,
            tradable: true,
        },
    ]
}
