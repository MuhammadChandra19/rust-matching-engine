#![allow(dead_code)]
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::core::log::{DoneLog, Log, MatchLog, OpenLog};
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
    /// This function attempts to match a given market order against the current limit orders.
    /// It iterates through the limit orders and performs the following steps:
    /// - If the size of the market order is greater than or equal to the size of a limit order,
    ///   the limit order is fully filled, and the remaining size of the market order is reduced.
    /// - If the size of the market order is smaller than a limit order, the market order is fully filled,
    ///   and the remaining size of the limit order is reduced.
    ///
    /// During the matching process, logs are generated for each match and for any remaining open orders.
    /// These logs include `MatchLog` for matched orders and `DoneLog` for orders that is filled after
    /// processing.
    ///
    /// # Arguments
    /// * `market_order` - A mutable reference to the market `Order` to be filled.
    ///
    /// # Returns
    /// * A `Vec<dyn Log>` containing logs of matches and filleds orders.
    ///
    /// # Behavior
    /// - Fully filled limit orders are removed from the order book.
    /// - Limit orders with remaining size after matching are retained.
    /// - The function generates logs for matched orders and done orders.
    ///
    /// # Logs
    /// * `MatchLog` is generated when a match occurs between a market order and a limit order.
    /// * `DoneLog` is generated for orders that remain in the order book after processing.
    pub(crate) fn fill_order(&mut self, market_order: &mut Order) -> Vec<dyn Log> {
        let mut logs: Vec<dyn Log> = vec![];
        for limit_order  in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    logs.push(MatchLog::new(
                            limit_order.next_log_seq(),
                            "PAIR".to_string(),
                            market_order.clone().id,
                            limit_order.clone().id,
                            limit_order.price,
                            market_order.size
                        )
                    );

                    limit_order.size = dec!(0);  // Fully filled limit order.
                }
                false => {
                    limit_order.size -= market_order.size;
                    logs.push(MatchLog::new(
                            limit_order.next_log_seq(),
                            "PAIR".to_string(),
                            market_order.clone().id,
                            limit_order.clone().id,
                            limit_order.price,
                            market_order.size
                        )
                    );
                    market_order.size = dec!(0);  // Fully filled market order.
                }
            }
        }

        // Retain only the limit orders that still have size left.
        self.orders.retain(|mut order| {
            logs.push(DoneLog::new(
                order.next_log_seq(),
                "PAIR".to_string(),
                order.clone().id,
                order.price,
                dec!(0),
                "FILLED".to_string(),
                order.clone().bid_or_ask
            ));
            order.size > dec!(0)
        });
        logs
    }

}
