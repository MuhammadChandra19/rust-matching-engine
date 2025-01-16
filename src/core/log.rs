#![allow(dead_code)]
use std::time::SystemTime;
use rust_decimal::Decimal;
use crate::core::order::BidOrAsk;

// Enum for log types
#[derive(Debug, Clone, Copy)]
enum LogType {
    Match,
    Open,
    Done,
}

// Trait for logs (equivalent to the abstract base class in C++)
pub(crate) trait Log {
    fn get_seq(&self) -> i64;
}

// Base structure for common fields
#[derive(Debug)]
struct Base {
    log_type: LogType,
    sequence: i64,
    pair: String,
    time: SystemTime,
}

impl Base {
    fn new(log_type: LogType, sequence: i64, pair: String, time: SystemTime) -> Self {
        Base {
            log_type,
            sequence,
            pair,
            time,
        }
    }
}

// Derived structure for ReceivedLog
#[derive(Debug)]
struct ReceivedLog {
    base: Base,
    order_id: String,
    size: f64,
    price: f64,
    order_type: i64,
}

impl ReceivedLog {
    fn new(sequence: i64, pair: String, order_id: String, size: f64, price: f64, order_type: i64) -> Self {
        ReceivedLog {
            base: Base::new(LogType::Done, sequence, pair, SystemTime::now()),
            order_id,
            size,
            price,
            order_type,
        }
    }
}

impl Log for ReceivedLog {
    fn get_seq(&self) -> i64 {
        self.base.sequence
    }
}

// Derived structure for OpenLog
#[derive(Debug)]
pub(crate) struct OpenLog {
    base: Base,
    order_id: String,
    size: Decimal,
    price: Decimal,
    bid_or_ask: BidOrAsk,
}

impl OpenLog {
    pub(crate) fn new(sequence: i64, pair: String, order_id: String, size: Decimal, price: Decimal, bid_or_ask: BidOrAsk) -> Self {
        OpenLog {
            base: Base::new(LogType::Open, sequence, pair, SystemTime::now()),
            order_id,
            size,
            price,
            bid_or_ask,
        }
    }
}

impl Log for OpenLog {
    fn get_seq(&self) -> i64 {
        self.base.sequence
    }
}

// Derived structure for DoneLog
#[derive(Debug)]
pub(crate) struct DoneLog {
    base: Base,
    order_id: String,
    price: Decimal,
    remaining_size: Decimal,
    reason: String,
    bid_or_ask: BidOrAsk,
}

impl DoneLog {
    pub(crate) fn new(sequence: i64, pair: String, order_id: String, price: Decimal, remaining_size: Decimal, reason: String, bid_or_ask: BidOrAsk) -> Self {
        DoneLog {
            base: Base::new(LogType::Done, sequence, pair, SystemTime::now()),
            order_id,
            price,
            remaining_size,
            reason,
            bid_or_ask,
        }
    }
}

impl Log for DoneLog {
    fn get_seq(&self) -> i64 {
        self.base.sequence
    }
}

// Derived structure for MatchLog
#[derive(Debug)]
pub struct MatchLog {
    base: Base,
    taker_order_id: String,
    maker_order_id: String,
    side: String,
    price: Decimal,
    size: Decimal,
}

impl MatchLog {
    pub fn new(sequence: i64, pair: String, taker_order_id: String, maker_order_id: String, price: Decimal, size: Decimal) -> Self {
        MatchLog {
            base: Base::new(LogType::Match, sequence, pair, SystemTime::now()),
            taker_order_id,
            maker_order_id,
            side: String::from(""),
            price,
            size,
        }
    }
}

impl Log for MatchLog {
    fn get_seq(&self) -> i64 {
        self.base.sequence
    }
}