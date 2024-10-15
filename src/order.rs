use std::{collections::BinaryHeap, cmp::Reverse, error::Error};

#[derive(Debug, Eq, PartialEq)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Eq, PartialEq)]
pub struct Order {
    pub id: usize,
    pub order_type: OrderType,
    pub price: u32,
    pub quantity: u32
}

impl Order {
    pub fn parse_input(input_str: &str) -> Result<Option<Self>, Box<dyn Error>> {
        let parts: Vec<&str> = input_str.split_whitespace().collect();

        if parts.len() != 7 {
            return Ok(None);
        }
        let order_type = match parts[1] {
            "Buy" => OrderType::Buy,
            "Sell" => OrderType::Sell,
            _ => return Ok(None),
        };

        let id: usize = parts[0].trim_end_matches(':').parse()?;
        let quantity: u32 = parts[2].parse()?;
        let price: u32 = parts[5].parse()?;

        Ok(Some(Self{id, order_type, price, quantity}))
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.price.cmp(&other.price).then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct OrderBook {
    heap: BinaryHeap<Reverse<Order>>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, order: Order) -> () {
        self.heap.push(Reverse(order));
    }
    
    pub fn pop(&mut self) -> Option<Order> {
        self.heap.pop().map(|Reverse(order)| order)
    }
}

pub struct Trade {
    pub buy_id: usize,
    pub sell_id: usize,
    pub price: u32, // this should be the sell price.
    pub quantity_traded: u32
}

impl Trade {
    pub fn make_trade(
        &self, mut buy_order: Order, mut sell_order: Order
    ) -> (Order, Order) {
        buy_order.quantity -= self.quantity_traded;
        sell_order.quantity -= self.quantity_traded;
        
        self.emit_trade();
        
        (buy_order, sell_order)
    }

    fn emit_trade(&self) -> () {
        println!(
            "Trade {} BTC @ {} USD between {} and {}",
            self.quantity_traded,
            self.price,
            self.buy_id,
            self.sell_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_parse_input_valid_buy() {
        let input = "1: Buy 10 BTC @ 120 USD";
        let order = Order::parse_input(input).unwrap().unwrap();

        assert_eq!(order.id, 1);
        assert_eq!(order.order_type, OrderType::Buy);
        assert_eq!(order.quantity, 10);
        assert_eq!(order.price, 120);
    }

    #[test]
    fn test_order_parse_input_valid_sell() {
        let input = "2: Sell 5 BTC @ 100 USD";
        let order = Order::parse_input(input).unwrap().unwrap();

        assert_eq!(order.id, 2);
        assert_eq!(order.order_type, OrderType::Sell);
        assert_eq!(order.quantity, 5);
        assert_eq!(order.price, 100);
    }

    #[test]
    fn test_order_parse_input_invalid() {
        let input = "Invalid input string";
        let order = Order::parse_input(input).unwrap();
        assert!(order.is_none());
    }

    #[test]
    fn test_order_book_push_and_pop() {
        let mut order_book = OrderBook::new();
        let order1 = Order { id: 1, order_type: OrderType::Buy, price: 120, quantity: 10 };
        let order2 = Order { id: 2, order_type: OrderType::Sell, price: 100, quantity: 5 };

        order_book.push(order1);
        order_book.push(order2);

        let popped_order = order_book.pop().unwrap();
        assert_eq!(popped_order.id, 2);
        assert_eq!(popped_order.price, 100);
    }

    #[test]
    fn test_trade_make_trade() {
        let buy_order = Order { id: 1, order_type: OrderType::Buy, price: 120, quantity: 10 };
        let sell_order = Order { id: 2, order_type: OrderType::Sell, price: 100, quantity: 5 };
        let trade = Trade { buy_id: 1, sell_id: 2, price: 100, quantity_traded: 5 };

        let (updated_buy_order, updated_sell_order) = trade.make_trade(buy_order, sell_order);

        assert_eq!(updated_buy_order.quantity, 5);
        assert_eq!(updated_sell_order.quantity, 0);
    }
}
