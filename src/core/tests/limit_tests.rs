#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;
    use crate::core::limit::Limit;
    use crate::core::order::{BidOrAsk, Order};

    // Helper function to create a dummy order
    fn create_order(id: String, size: Decimal, price: Decimal, bid_or_ask: BidOrAsk) -> Order {
        Order::new(id, bid_or_ask, price, size)
    }

    // Test for the `new` method
    #[test]
    fn test_new_limit() {
        let price = dec!(100);
        let limit = Limit::new(price);
        assert_eq!(limit.price, price);
        assert!(limit.orders.is_empty());
    }

    // Test for the `add_order` method
    #[test]
    fn test_add_order() {
        let mut limit = Limit::new(dec!(100));
        let order = create_order("1".to_string(), dec!(10), dec!(100), BidOrAsk::Ask);
        let _open_log = limit.add_order(order.clone());

        // Assert that the order was added and log generated
        assert_eq!(limit.orders.len(), 1);
        assert_eq!(limit.orders[0].id, "1");
    }

    // Test for the `delete_order` method
    #[test]
    fn test_delete_order() {
        let mut limit = Limit::new(dec!(100));
        let order = create_order("1".to_string(), dec!(10), dec!(100), BidOrAsk::Ask);
        limit.add_order(order.clone());

        let _done_log = limit.delete_order("1".to_string());

        // Assert that the order was deleted
        assert!(limit.orders.is_empty());
    }

    // Test for the `fill_order` method
    #[test]
    fn test_fill_order() {
        let mut limit = Limit::new(dec!(100));
        let order1 = create_order("1".to_string(), dec!(10), dec!(100), BidOrAsk::Ask);
        let order2 = create_order("2".to_string(), dec!(5), dec!(100), BidOrAsk::Ask);

        limit.add_order(order1.clone());
        limit.add_order(order2.clone());

        let mut market_order = create_order("3".to_string(), dec!(12), dec!(100), BidOrAsk::Bid);
        let match_results = limit.fill_order(&mut market_order);

        // Assert that the market order is filled correctly
        assert_eq!(market_order.size, dec!(0)); // Fully filled
        assert_eq!(match_results.len(), 2); // Two match results
        assert_eq!(match_results[0].size_filled, dec!(10)); // First match filled 10
        assert_eq!(match_results[1].size_filled, dec!(2));  // Second match filled 2

        // Assert that the orders in the limit are updated correctly
        assert_eq!(limit.orders.len(), 1); // One limit order remains
        assert_eq!(limit.orders[0].size, dec!(3)); // Remaining size is 3
    }
}
