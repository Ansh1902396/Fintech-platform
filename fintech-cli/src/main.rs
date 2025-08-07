use std::io;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Serialize, Deserialize)]
struct Order {
    price: f64,
    amount: f64,
    side: Side,
    signer: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Orderbook {
    // Add fields as needed
    orders: Vec<Order>,
}

fn read_from_stdin(label: &str) -> String {
    let mut buffer = String::new();
    println!("{}", label);
    io::stdin()
        .read_line(&mut buffer)
        .expect("Couldn't read from stdin");
    buffer.trim().to_owned()
}

#[tokio::main]
async fn main() {
    println!("Hello, accounting world!");

    let client = Client::new();

    loop {
        let input = read_from_stdin(
            "Choose operation [deposit, withdraw, send, print, orderbook, order, quit], confirm with return:",
        );
        match input.as_str() {
            "deposit" => {
                let account = read_from_stdin("Account:");

                let raw_amount: Result<f64, _> = read_from_stdin("Amount:").parse();
                if let Ok(amount) = raw_amount {
                    let _ = client.post("http://localhost:3030/deposit")
                        .json(&serde_json::json!({
                            "account": account,
                            "amount": amount
                        }))
                        .send()
                        .await;
                    println!("Deposited {} into account '{}'", amount, account)
                } else {
                    eprintln!("Not a number: '{:?}'", raw_amount);
                }
            }
            "withdraw" => {
                let account = read_from_stdin("Account:");
                let raw_amount: Result<f64, _> = read_from_stdin("Amount:").parse();
                if let Ok(amount) = raw_amount {
                    let _ = client.post("http://localhost:3030/withdraw")
                        .json(&serde_json::json!({
                            "account": account,
                            "amount": amount
                        }))
                        .send()
                        .await;
                    println!("Withdrew {} from account '{}'", amount, account)
                } else {
                    eprintln!("Not a number: '{:?}'", raw_amount);
                }
            }
            "send" => {
                let sender = read_from_stdin("Sender Account:");
                let recipient = read_from_stdin("Recipient Account:");
                let raw_amount: Result<f64, _> = read_from_stdin("Amount:").parse();
                if let Ok(amount) = raw_amount {
                    let _ = client.post("http://localhost:3030/send")
                        .json(&serde_json::json!({
                            "sender": sender,
                            "recipient": recipient,
                            "amount": amount
                        }))
                        .send()
                        .await;
                } else {
                    eprintln!("Not a number: '{:?}'", raw_amount);
                }
            }
            "order" => {
                let raw_price: Result<f64, _> = read_from_stdin("Price:").parse();
                let raw_amount: Result<f64, _> = read_from_stdin("Amount:").parse();
                let side_input = read_from_stdin("Side (buy/sell):");
                let signer = read_from_stdin("Signer:");

                if let (Ok(price), Ok(amount)) = (raw_price, raw_amount) {
                    let side = match side_input.as_str() {
                        "buy" => Side::Buy,
                        "sell" => Side::Sell,
                        _ => {
                            eprintln!("Invalid side: '{}'", side_input);
                            continue;
                        }
                    };
                    let order = Order {
                        price,
                        amount,
                        side,
                        signer,
                    };
                    // Send order to server instead of using local trading_platform
                    let response = client.post("http://localhost:3030/order")
                        .json(&order)
                        .send()
                        .await;
                    match response {
                        Ok(res) => {
                            if res.status().is_success() {
                                println!("Order processed successfully");
                            } else {
                                eprintln!("Error processing order: {}", res.status());
                            }
                        },
                        Err(e) => eprintln!("Error sending order: {:?}", e),
                    }
                } else {
                    eprintln!("Invalid price or amount");
                }
            }
            "orderbook" => {
                let response = client.get("http://localhost:3030/orderbook")
                    .send()
                    .await;
                
                let orderbook = match response {
                    Ok(res) => res.json::<Orderbook>().await.unwrap_or_else(|e| {
                        eprintln!("Error parsing orderbook: {:?}", e);
                        Orderbook::default()
                    }),
                    Err(e) => {
                        eprintln!("Error fetching orderbook: {:?}", e);
                        Orderbook::default()
                    }
                };
                println!("Orderbook: {:?}", orderbook);
            }
            "print" => {
                // Fetch accounts from server instead of using local trading_platform
                let response = client.get("http://localhost:3030/accounts")
                    .send()
                    .await;
                
                match response {
                    Ok(res) => {
                        if let Ok(accounts) = res.text().await {
                            println!("The ledger: {}", accounts);
                        } else {
                            eprintln!("Error reading accounts response");
                        }
                    },
                    Err(e) => eprintln!("Error fetching accounts: {:?}", e),
                }
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