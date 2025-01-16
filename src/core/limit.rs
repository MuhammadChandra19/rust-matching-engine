#![allow(dead_code)]
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::core::log::{DoneLog, OpenLog};
use crate::core::match_result::MatchResult;
use crate::core::order::Order;

/// A struct representing a limit order, which consists of a price and a list of associated orders.
///
/// The `Limit` struct is used to manage a collection of orders that share the same price. It provides
/// functionality to add new orders, delete existing ones, and compute the total volume of orders at
/// this price level.
#[derive(Debug)]
pub struct Limit {
    pub(crate) price: Decimal,        // The price for this limit order.
    pub(crate) orders: Vec<Order>,    // A list of orders associated with this limit price.
}

impl Limit {
    /// Creates a new `Limit` with a specified price and an empty list of orders.
    ///
    /// # Arguments
    /// * `price` - A `Decimal` representing the price of the limit order.
    ///
    /// # Returns
    /// * A new `Limit` struct with the given price and an empty order list.
    pub fn new(price: Decimal) -> Self {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    /// Calculates the total volume of all orders associated with this limit order.
    ///
    /// The total volume is computed by summing the size of each order in the `orders` list.
    ///
    /// # Returns
    /// * The total volume as a `Decimal`.
    fn total_volume(&self) -> Decimal {
        self.orders
            .iter()
            .map(|order| order.size)  // Summing the size of each order.
            .reduce(|a, b| a + b)    // Reducing the sizes to get the total volume.
            .unwrap()                 // This will panic if the list is empty.
    }

    /// Adds a new order to the limit order book and generates an `OpenLog` entry.
    ///
    /// The order is pushed into the `orders` list, and an `OpenLog` is generated for the action.
    ///
    /// # Arguments
    /// * `order` - An `Order` object to be added to the limit order book.
    ///
    /// # Returns
    /// * An `OpenLog` representing the addition of the order.
    pub(crate) fn add_order(&mut self, mut order: Order) -> OpenLog {
        self.orders.push(order.clone());  // Adding the order to the list.
        OpenLog::new(
            order.next_log_seq(),  // Sequence number for the log.
            "PAIR".to_string(),     // Log type.
            order.id,              // Order ID.
            order.size,            // Order size.
            order.price,           // Order price.
            order.bid_or_ask       // Order type (bid or ask).
        )
    }

    /// Deletes an order by its ID and generates a `DoneLog` entry for the removal.
    ///
    /// The order is removed from the `orders` list, and a `DoneLog` is generated to indicate the
    /// deletion.
    ///
    /// # Arguments
    /// * `id` - A `String` representing the ID of the order to be deleted.
    ///
    /// # Returns
    /// * A `DoneLog` representing the deletion of the order.
    pub(crate) fn delete_order(&mut self, id: String) -> DoneLog {
        let mut order = self.orders.iter()
            .find(|&order| order.id == id)  // Find the order by its ID.
            .unwrap()                        // Will panic if no matching order is found.
            .clone();
        self.orders.retain(|order| order.id != id);  // Remove the order from the list.

        DoneLog::new(
            order.next_log_seq(),  // Sequence number for the log.
            "PAIR".to_string(),     // Log type.
            id,                     // The ID of the deleted order.
            order.price,            // The price of the deleted order.
            order.size,             // The size of the deleted order.
            "DELETED".to_string(),  // Status indicating the order was deleted.
            order.bid_or_ask       // The type of the deleted order (bid or ask).
        )
    }

    /// Fills a market order by matching it with limit orders at this price level.
    ///
    /// This function attempts to fill the provided market order by matching it against the limit
    /// orders in the `Limit`'s order book. If the size of the market order is greater than or equal
    /// to the size of a limit order, the limit order is fully filled, and the market order's size is
    /// reduced accordingly. If the size of the market order is smaller than a limit order, the market
    /// order is completely filled, and the limit order's size is reduced.
    ///
    /// The function continues to match the market order with limit orders until the market order is
    /// fully filled or no matching limit orders remain. It returns a list of `MatchResult` that
    /// describes each match made during the process.
    ///
    /// # Arguments
    /// * `market_order` - A mutable reference to the `Order` representing the market order to be filled.
    ///
    /// # Returns
    /// * A `Vec<MatchResult>` representing the result of each match, including the matched ask and
    ///   bid orders, the price, and the size of the match.
    pub(crate) fn fill_order(&mut self, market_order: &mut Order) -> Vec<MatchResult> {
        let mut match_result: Vec<MatchResult> = vec![];
        for limit_order  in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    match_result.push(MatchResult {
                        ask: limit_order.clone(),
                        bid: market_order.clone(),
                        price: self.price,
                        size_filled: limit_order.size,
                    });
                    limit_order.size = dec!(0);  // Fully filled limit order.
                }
                false => {
                    limit_order.size -= market_order.size;
                    match_result.push(MatchResult {
                        ask: limit_order.clone(),
                        bid: market_order.clone(),
                        price: self.price,
                        size_filled: market_order.size,
                    });
                    market_order.size = dec!(0);  // Fully filled market order.
                }
            }
        }

        // Retain only the limit orders that still have size left.
        self.orders.retain(|order| order.size > dec!(0));
        match_result
    }

}
