mod tests_order_book {
    use rust_decimal_macros::dec;
    use crate::core::order::{Order, BidOrAsk};
    use crate::core::order_book::OrderBook;

    #[test]
    fn test_new_order_book() {
        let order_book = OrderBook::new();

        assert!(order_book.asks.is_empty(), "Expected asks to be empty");
        assert!(order_book.bids.is_empty(), "Expected bids to be empty");
    }

    #[test]
    fn test_add_limit_order_bid() {
        let mut order_book = OrderBook::new();
        let price = dec!(100.0);
        let  order = Order::new("1".to_string(),  BidOrAsk::Bid, price,dec!(10), );

        let log = order_book.add_limit_order(price, order);

        assert_eq!(order_book.bids.len(), 1, "Expected one bid limit");
        assert!(order_book.bids.contains_key(&price), "Expected bid price level to exist");
        assert_eq!(log.order_id, "1", "Expected log order ID to match");
    }

    #[test]
    fn test_add_limit_order_ask() {
        let mut order_book = OrderBook::new();
        let price = dec!(200.0);
        let order = Order::new("2".to_string(),BidOrAsk::Ask,price,  dec!(5.0), );

        let log = order_book.add_limit_order(price, order);

        assert_eq!(order_book.asks.len(), 1, "Expected one ask limit");
        assert!(order_book.asks.contains_key(&price), "Expected ask price level to exist");
        assert_eq!(log.order_id, "2", "Expected log order ID to match");
    }

    #[test]
    fn test_fill_market_order_bid() {
        let mut order_book = OrderBook::new();
        let price = dec!(100.0);
        let limit_order = Order::new("1".to_string(), BidOrAsk::Ask,price,  dec!(10.0), );
        order_book.add_limit_order(price, limit_order);

        let mut market_order = Order::new("2".to_string(),BidOrAsk::Bid,price, dec!(5.0),  );
        let logs = order_book.fill_market_order(&mut market_order);

        assert_eq!(logs.len(), 1, "Expected one log for a partial fill");
        assert!(market_order.is_filled(), "Expected market order to be fully filled");
        assert_eq!(order_book.asks[&price].orders.len(), 1, "Expected limit order to remain with reduced size");
    }

    #[test]
    fn test_fill_market_order_ask() {
        let mut order_book = OrderBook::new();
        let price = dec!(150.0);
        let limit_order = Order::new("1".to_string(), BidOrAsk::Bid,price, dec!(20.0));
        order_book.add_limit_order(price, limit_order);

        let mut market_order = Order::new("2".to_string(),  BidOrAsk::Ask, price,dec!(15.0));
        let logs = order_book.fill_market_order(&mut market_order);

        assert_eq!(logs.len(), 1, "Expected one log for a partial fill");
        assert!(market_order.is_filled(), "Expected market order to be fully filled");
        assert_eq!(order_book.bids[&price].orders.len(), 1, "Expected limit order to remain with reduced size");
    }

    #[test]
    fn test_partial_fill_market_order() {
        let mut order_book = OrderBook::new();
        let price = dec!(120.0);
        let limit_order1 = Order::new("1".to_string(), BidOrAsk::Ask, dec!(10.0), price);
        let limit_order2 = Order::new("2".to_string(),  BidOrAsk::Ask,dec!(10.0), price);
        order_book.add_limit_order(price, limit_order1);
        order_book.add_limit_order(price, limit_order2);

        let mut market_order = Order::new("3".to_string(), BidOrAsk::Bid,dec!(15.0), price, );
        let logs = order_book.fill_market_order(&mut market_order);

        assert_eq!(logs.len(), 3, "Expected two logs for partial fills");
        assert!(market_order.is_filled(), "Expected market order to be fully filled");
        assert_eq!(order_book.asks[&price].orders.len(), 1, "Expected one remaining limit order");
    }

    #[test]
    fn test_fully_fill_limit_orders() {
        let mut order_book = OrderBook::new();
        let price = dec!(110.0);
        let limit_order1 = Order::new("1".to_string(),  BidOrAsk::Ask,dec!(10.0), price,);
        let limit_order2 = Order::new("2".to_string(),  BidOrAsk::Ask, dec!(10.0), price,);
        order_book.add_limit_order(price, limit_order1);
        order_book.add_limit_order(price, limit_order2);

        let mut market_order = Order::new("3".to_string(), BidOrAsk::Bid,dec!(20.0), price, );
        let logs = order_book.fill_market_order(&mut market_order);

        assert_eq!(logs.len(), 3, "Expected two logs for complete fills");
        assert!(market_order.is_filled(), "Expected market order to be fully filled");
    }

}