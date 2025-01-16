use std::collections::HashMap;
use rust_decimal::Decimal;
use crate::core::limit::Limit;
use crate::core::log::{Log, OpenLog};
use crate::core::order::{BidOrAsk, Order};

/// Represents an order book containing bid and ask limits.
/// The `OrderBook` struct manages buy and sell orders, organized by price levels.
#[derive(Debug)]
pub struct OrderBook {
    pub(crate) asks: HashMap<Decimal, Limit>, // Map of price levels to ask (sell) limits.
    pub(crate) bids: HashMap<Decimal, Limit>, // Map of price levels to bid (buy) limits.
}

impl OrderBook {
    /// Creates a new, empty order book with no bid or ask limits.
    pub fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    /// Fills a market order by matching it with the opposing limit orders.
    ///
    /// This function identifies the appropriate limits (asks for bid orders and bids for ask orders)
    /// and iteratively attempts to fill the market order by matching it with the orders in these limits.
    ///
    /// # Arguments
    /// * `market_order` - A mutable reference to the market order that needs to be filled.
    ///
    /// # Returns
    /// * A `Vec<Box<dyn Log>>` containing logs for matches and open orders.
    pub fn fill_market_order(&mut self, market_order: &mut Order) -> Vec<Box<dyn Log>> {
        let mut logs: Vec<Box<dyn Log>> = vec![];
        let mut limits = match market_order.bid_or_ask {
            BidOrAsk::Bid => self.ask_limits(), // Bids consume asks.
            BidOrAsk::Ask => self.bid_limits(), // Asks consume bids.
        };

        // Collect indices of limits to remove
        let mut remove_indices = Vec::new();
        for (index, limit_order) in limits.iter_mut().enumerate() {
            let result = limit_order.fill_order(market_order);

            if limit_order.orders.is_empty() {
                remove_indices.push(index);
            }

            logs.extend(result); // Collect logs for matches and open orders.

            if market_order.is_filled() {
                break; // Stop once the market order is completely filled.
            }
        }

        // Remove the limits after the iteration
        for index in remove_indices.iter().rev() {
            limits.remove(*index);
        }

        logs
    }

    /// Retrieves all ask (sell) limits, sorted by the cheapest price first.
    ///
    /// This function sorts the ask limits in ascending order of price, which is required
    /// for processing bid (buy) orders to match the lowest-priced sell orders.
    ///
    /// # Returns
    /// * A vector of mutable references to `Limit` sorted by price.
    pub fn ask_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.asks.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| a.price.cmp(&b.price));
        limits
    }

    /// Retrieves all bid (buy) limits, sorted by the highest price first.
    ///
    /// This function sorts the bid limits in descending order of price, which is required
    /// for processing ask (sell) orders to match the highest-priced buy orders.
    ///
    /// # Returns
    /// * A vector of mutable references to `Limit` sorted by price.
    pub fn bid_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.bids.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| b.price.cmp(&a.price));
        limits
    }

    /// Adds a new limit order to the order book.
    ///
    /// This function either creates a new `Limit` at the specified price level or
    /// appends the order to an existing limit. The operation depends on whether the
    /// limit already exists in the bid or ask map.
    ///
    /// # Arguments
    /// * `price` - The price level for the order.
    /// * `order` - The `Order` to be added to the limit.
    ///
    /// # Returns
    /// * An `OpenLog` containing information about the added limit order.
    pub fn add_limit_order(&mut self, price: Decimal, mut order: Order) -> OpenLog {
        let log = OpenLog::new(
            order.next_log_seq(),
            "PAIR".to_string(),
            order.id.clone(),
            order.size,
            price,
            order.bid_or_ask.clone(),
        );

        match order.bid_or_ask {
            BidOrAsk::Bid => match self.bids.get_mut(&price) {
                Some(limit) => {
                    limit.add_order(order); // Append order to existing bid limit.
                }
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order); // Create a new bid limit.
                    self.bids.insert(price, limit);
                }
            },
            BidOrAsk::Ask => match self.asks.get_mut(&price) {
                Some(limit) => {
                    limit.add_order(order); // Append order to existing ask limit.
                }
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order); // Create a new ask limit.
                    self.asks.insert(price, limit);
                }
            },
        }

        log
    }
}
