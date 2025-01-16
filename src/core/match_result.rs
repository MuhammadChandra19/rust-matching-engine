#![allow(dead_code)]
use rust_decimal::Decimal;
use crate::core::order::Order;

#[derive(Debug)]
pub struct MatchResult {
    pub ask: Order,
    pub bid: Order,

    pub(crate) size_filled: Decimal,
    pub(crate) price: Decimal
}

