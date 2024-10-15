mod order;
use std::{io, cmp::min};
use order::{Order, OrderBook, OrderType, Trade};


fn main() {
    let stdin = io::stdin();
    let mut order_book = OrderBook::new();
    let mut id_counter = 1;
    
    loop {
        let mut stdin_buffer = String::new();
        stdin.read_line(&mut stdin_buffer).unwrap();

        match Order::parse_input(&stdin_buffer) {
            Ok(Some(mut parsed_order)) => {
                if parsed_order.id != id_counter {
                    eprintln!("Invalid order number, should be order {}", id_counter);
                    continue;
                }
                match parsed_order.order_type {
                    OrderType::Buy => {
                        while parsed_order.quantity > 0 {
                            match order_book.pop() {
                                Some(mut popped_sell_order) => {
                                    if popped_sell_order.price <= parsed_order.price {
                                        let trade = Trade {
                                            buy_id: parsed_order.id,
                                            sell_id: popped_sell_order.id,
                                            price: popped_sell_order.price,
                                            quantity_traded: min(
                                                parsed_order.quantity,
                                                popped_sell_order.quantity
                                            ),
                                        };
                                        
                                        (parsed_order, popped_sell_order) = trade.make_trade(
                                            parsed_order,
                                            popped_sell_order
                                        );

                                        if popped_sell_order.quantity > 0 {
                                            order_book.push(popped_sell_order);
                                        }
                                        if parsed_order.quantity == 0 {
                                            println!("Buy order fulfilled");
                                            break
                                        }
                                    }
                                    else {
                                        println!("Unable to fulfil buy order at requested price");
                                    }
                                }
                                None => {
                                    println!("\nOrder book is empty");
                                    if parsed_order.quantity > 0 {
                                        println!(
                                            "Buy order {} was not completely fulfilled with \
                                            a quantity of {} remaining",
                                            parsed_order.id,
                                            parsed_order.quantity
                                        )
                                    }
                                    println!("Waiting for more sell orders...\n");
                                    break;
                                }
                            }
                        }
                    },
                    OrderType::Sell => {
                        order_book.push(parsed_order);
                    },
                }
                id_counter += 1;
            }
            Ok(None) => eprintln!(
                "Input incorrectly formatted. Input should be formatted as so: \
                \"{{id}}: {{Buy|Sell}} {{quantity}} BTC @ {{price}}\""
            ),
            Err(e) => eprintln!("Error parsing order: {}", e),
        }
    }
}
