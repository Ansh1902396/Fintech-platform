use std::io;
mod accounting;
mod core;
mod errors;
mod tx;
mod trading_platform;
use crate::accounting::Accounts;

fn read_from_stdin(label: &str) -> String {
    let mut buffer = String::new();
    println!("{}", label);
    io::stdin()
        .read_line(&mut buffer)
        .expect("Couldn't read from stdin");
    buffer.trim().to_owned()
}

fn main() {
    println!("Hello, accounting world!");

    let mut trading_platform = trading_platform::TradingPlatform::new();
    loop {
        let input = read_from_stdin(
            "Choose operation [deposit, withdraw, send, print, orderbook, order, quit], confirm with return:",
        );
        match input.as_str() {
            "deposit" => {
                let account = read_from_stdin("Account:");

                let raw_amount = read_from_stdin("Amount:").parse();
                if let Ok(amount) = raw_amount {
                    let _ = trading_platform.deposit(&account, amount);
                    println!("Deposited {} into account '{}'", amount, account)
                } else {
                    eprintln!("Not a number: '{:?}'", raw_amount);
                }
            }
            "withdraw" => {
                let account = read_from_stdin("Account:");
                let raw_amount = read_from_stdin("Amount:").parse();
                if let Ok(amount) = raw_amount {
                    let _ = trading_platform.withdraw(&account, amount);
                } else {
                    eprintln!("Not a number: '{:?}'", raw_amount);
                }
            }
            "send" => {
                let sender = read_from_stdin("Sender Account:");
                let recipient = read_from_stdin("Recipient Account:");
                let raw_amount = read_from_stdin("Amount:").parse();
                if let Ok(amount) = raw_amount {
                    let _ = trading_platform.send(&sender, &recipient, amount);
                } else {
                    eprintln!("Not a number: '{:?}'", raw_amount);
                }
            }
            "order" => {
                let raw_price = read_from_stdin("Price:").parse();
                let raw_amount = read_from_stdin("Amount:").parse();
                let side_input = read_from_stdin("Side (buy/sell):");
                let signer = read_from_stdin("Signer:");

                if let (Ok(price), Ok(amount)) = (raw_price, raw_amount) {
                    let side = match side_input.as_str() {
                        "buy" => core::Side::Buy,
                        "sell" => core::Side::Sell,
                        _ => {
                            eprintln!("Invalid side: '{}'", side_input);
                            continue;
                        }
                    };
                    let order = core::Order {
                        price,
                        amount,
                        side,
                        signer,
                    };
                    match trading_platform.order(order) {
                        Ok(receipt) => println!("Order processed: {:?}", receipt),
                        Err(e) => eprintln!("Error processing order: {:?}", e),
                    }
                } else {
                    eprintln!("Invalid price or amount");
                }
            }
            "orderbook" => {
                let orderbook = trading_platform.orderbook();
                println!("Orderbook: {:?}", orderbook);
            }
            "print" => {
                println!("The ledger: {:?}", trading_platform.accounts);
            }
            "quit" => {
                println!("Quitting...");
                break;
            }
            _ => {
                eprintln!("Invalid option: '{}'", input);
            }
        }
    }
}
