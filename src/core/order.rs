#![allow(dead_code)]
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub(crate) id: String,
    pub(crate) price: Decimal,
    pub size: Decimal,
    pub(crate) bid_or_ask: BidOrAsk,
    created_at: SystemTime,
}

impl Order {
    pub fn new(id: String, bid_or_ask: BidOrAsk, price: Decimal, size: Decimal) -> Self {
        Order {
            id,
            bid_or_ask,
            price,
            size,
            created_at: SystemTime::now(),
        }
    }

    pub fn is_filled(&self) -> bool {
        self.size == dec!(0)
    }
}
