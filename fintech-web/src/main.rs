mod accounting;
mod core;
use  fintech_common::{errors , tx};
mod trading_platform;

// fn read_from_stdin(label: &str) -> String {
//     let mut buffer = String::new();
//     println!("{}", label);
//     io::stdin()
//         .read_line(&mut buffer)
//         .expect("Couldn't read from stdin");
//     buffer.trim().to_owned()
// }

// fn main() {
//     println!("Hello, accounting world!");

//     let mut trading_platform = trading_platform::TradingPlatform::new();
//     loop {
//         let input = read_from_stdin(
//             "Choose operation [deposit, withdraw, send, print, orderbook, order, quit], confirm with return:",
//         );
//         match input.as_str() {
//             "deposit" => {
//                 let account = read_from_stdin("Account:");

//                 let raw_amount = read_from_stdin("Amount:").parse();
//                 if let Ok(amount) = raw_amount {
//                     let _ = trading_platform.deposit(&account, amount);
//                     println!("Deposited {} into account '{}'", amount, account)
//                 } else {
//                     eprintln!("Not a number: '{:?}'", raw_amount);
//                 }
//             }
//             "withdraw" => {
//                 let account = read_from_stdin("Account:");
//                 let raw_amount = read_from_stdin("Amount:").parse();
//                 if let Ok(amount) = raw_amount {
//                     let _ = trading_platform.withdraw(&account, amount);
//                 } else {
//                     eprintln!("Not a number: '{:?}'", raw_amount);
//                 }
//             }
//             "send" => {
//                 let sender = read_from_stdin("Sender Account:");
//                 let recipient = read_from_stdin("Recipient Account:");
//                 let raw_amount = read_from_stdin("Amount:").parse();
//                 if let Ok(amount) = raw_amount {
//                     let _ = trading_platform.send(&sender, &recipient, amount);
//                 } else {
//                     eprintln!("Not a number: '{:?}'", raw_amount);
//                 }
//             }
//             "order" => {
//                 let raw_price = read_from_stdin("Price:").parse();
//                 let raw_amount = read_from_stdin("Amount:").parse();
//                 let side_input = read_from_stdin("Side (buy/sell):");
//                 let signer = read_from_stdin("Signer:");

//                 if let (Ok(price), Ok(amount)) = (raw_price, raw_amount) {
//                     let side = match side_input.as_str() {
//                         "buy" => core::Side::Buy,
//                         "sell" => core::Side::Sell,
//                         _ => {
//                             eprintln!("Invalid side: '{}'", side_input);
//                             continue;
//                         }
//                     };
//                     let order = core::Order {
//                         price,
//                         amount,
//                         side,
//                         signer,
//                     };
//                     match trading_platform.order(order) {
//                         Ok(receipt) => println!("Order processed: {:?}", receipt),
//                         Err(e) => eprintln!("Error processing order: {:?}", e),
//                     }
//                 } else {
//                     eprintln!("Invalid price or amount");
//                 }
//             }
//             "orderbook" => {
//                 let orderbook = trading_platform.orderbook();
//                 println!("Orderbook: {:?}", orderbook);
//             }
//             "print" => {
//                 println!("The ledger: {:?}", trading_platform.accounts);
//             }
//             "quit" => {
//                 println!("Quitting...");
//                 break;
//             }
//             _ => {
//                 eprintln!("Invalid option: '{}'", input);
//             }
//         }
//     }
// }

use warp::Filter;


#[tokio::main]
async fn main() {
   
    pretty_env_logger::init();

    let trading_platform = std::sync::Arc::new(std::sync::Mutex::new(trading_platform::TradingPlatform::new()));

    let routes = filters::deposit(trading_platform.clone())
        .or(filters::withdraw(trading_platform.clone()))
        .or(filters::send(trading_platform.clone()))
        .or(filters::order(trading_platform.clone()))
        .or(filters::orderbook(trading_platform.clone()))
        .or(filters::balance(trading_platform.clone()));

    println!("Starting server on http://127.0.0.1:3030");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}


mod filters {
    use fintech_common::core::types::{AccountBalanceRequest, AccountUpdateRequest, SendRequest, Order};
    use warp::Filter;
 
    pub fn deposit(tp: std::sync::Arc<std::sync::Mutex<crate::trading_platform::TradingPlatform>>) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
       warp::path!("deposit")
            .and(warp::post())
            .and(json_body::<AccountUpdateRequest>())
            .and(with_trading_platform(tp))
            .and_then(|req: AccountUpdateRequest, tp| crate::handlers::deposit(tp, req))
    }

    pub fn withdraw(tp: std::sync::Arc<std::sync::Mutex<crate::trading_platform::TradingPlatform>>) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
       warp::path!("withdraw")
            .and(warp::post())
            .and(json_body::<AccountUpdateRequest>())
            .and(with_trading_platform(tp))
            .and_then(|req: AccountUpdateRequest, tp| crate::handlers::withdraw(tp, req))
    }

    pub fn send(tp: std::sync::Arc<std::sync::Mutex<crate::trading_platform::TradingPlatform>>) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
       warp::path!("send")
            .and(warp::post())
            .and(json_body::<SendRequest>())
            .and(with_trading_platform(tp))
            .and_then(|req: SendRequest, tp| crate::handlers::send(tp, req))
    }

    pub fn order(tp: std::sync::Arc<std::sync::Mutex<crate::trading_platform::TradingPlatform>>) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
       warp::path!("order")
            .and(warp::post())
            .and(json_body::<Order>())
            .and(with_trading_platform(tp))
            .and_then(|req: Order, tp| crate::handlers::order(tp, req))
    }

    pub fn orderbook(tp: std::sync::Arc<std::sync::Mutex<crate::trading_platform::TradingPlatform>>) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
       warp::path!("orderbook")
            .and(warp::get())
            .and(with_trading_platform(tp))
            .and_then(|tp| crate::handlers::orderbook(tp))
    }

    pub fn balance(tp: std::sync::Arc<std::sync::Mutex<crate::trading_platform::TradingPlatform>>) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
       warp::path!("balance")
            .and(warp::post())
            .and(json_body::<AccountBalanceRequest>())
            .and(with_trading_platform(tp))
            .and_then(|req: AccountBalanceRequest, tp| crate::handlers::balance(tp, req))
    }

    fn with_trading_platform(tp: std::sync::Arc<std::sync::Mutex<crate::trading_platform::TradingPlatform>>) -> impl warp::Filter<Extract = (std::sync::Arc<std::sync::Mutex<crate::trading_platform::TradingPlatform>>,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || tp.clone())
    }


    fn json_body<T: serde::de::DeserializeOwned + Send>() -> impl warp::Filter<Extract = (T,), Error = warp::Rejection> + Clone {
         warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
    
}

mod handlers {
    use std::convert::Infallible;
    use fintech_common::core::types::{AccountBalanceRequest, AccountUpdateRequest, Order, SendRequest};

    use crate::trading_platform::TradingPlatform;
    use std::sync::{Arc, Mutex};


    pub async fn deposit(tp : Arc<Mutex<TradingPlatform>> , req: AccountUpdateRequest ) -> Result<impl warp::Reply ,Infallible> {
        let mut platform = tp.lock().unwrap();
        match platform.deposit(&req.account, req.amount) {
            Ok(_) => Ok(warp::reply::json(&"Deposit successful")),
            Err(e) => Ok(warp::reply::json(&format!("Error: {:?}", e))),
        }
    }


    pub async fn withdraw(tp : Arc<Mutex<TradingPlatform>> , req: AccountUpdateRequest ) -> Result<impl warp::Reply ,Infallible> {
        let mut platform = tp.lock().unwrap();
        match platform.withdraw(&req.account, req.amount) {
            Ok(_) => Ok(warp::reply::json(&"Withdrawal successful")),
            Err(e) => Ok(warp::reply::json(&format!("Error: {:?}", e))),
        }
    }

    pub async fn send(tp : Arc<Mutex<TradingPlatform>> , req: SendRequest ) -> Result<impl warp::Reply ,Infallible> {
        let mut platform = tp.lock().unwrap();
        match platform.send(&req.sender, &req.recipient, req.amount) {
            Ok(_) => Ok(warp::reply::json(&"Transfer successful")),
            Err(e) => Ok(warp::reply::json(&format!("Error: {:?}", e))),
        }
    }

    pub async fn order(tp : Arc<Mutex<TradingPlatform>> , req:Order ) -> Result<impl warp::Reply ,Infallible> {
        let mut platform = tp.lock().unwrap();
        match platform.order(req) {
            Ok(receipt) => Ok(warp::reply::json(&receipt)),
            Err(e) => Ok(warp::reply::json(&format!("Error processing order: {:?}", e))),
        }
    }


    //getter function for orderbook
    pub async fn orderbook(tp : Arc<Mutex<TradingPlatform>>) -> Result<impl warp::Reply, Infallible> {
        let platform = tp.lock().unwrap();
        let orderbook = platform.orderbook();
        Ok(warp::reply::json(&orderbook))
    }


    pub async fn balance(tp : Arc<Mutex<TradingPlatform>> , req : AccountBalanceRequest) -> Result<impl warp::Reply, Infallible> {
        let mut  platform = tp.lock().unwrap();
        match platform.balance_of(&req.account) {
            Ok(balance) => Ok(warp::reply::json(&balance)),
           Err(e) => Ok(warp::reply::json(&format!("Error: {:?}", e))),
        }
    }
}