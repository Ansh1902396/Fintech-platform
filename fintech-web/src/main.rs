mod accounting;
mod core;
use  fintech_common::{errors , tx};
mod trading_platform;
use warp::Filter;


#[tokio::main]
async fn main() {
   
    pretty_env_logger::init();
    log::info!("Starting Fintech Trading Platform Server");

    let trading_platform = std::sync::Arc::new(std::sync::Mutex::new(trading_platform::TradingPlatform::new()));
    log::info!("Trading platform initialized");

    let routes = filters::deposit(trading_platform.clone())
        .or(filters::withdraw(trading_platform.clone()))
        .or(filters::send(trading_platform.clone()))
        .or(filters::order(trading_platform.clone()))
        .or(filters::orderbook(trading_platform.clone()))
        .or(filters::balance(trading_platform.clone()));

    log::info!("Routes configured");
    println!("Starting server on http://127.0.0.1:3030");
    log::info!("Server starting on http://127.0.0.1:3030");
    
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
    use log::{info, error};


    pub async fn deposit(tp : Arc<Mutex<TradingPlatform>> , req: AccountUpdateRequest ) -> Result<impl warp::Reply ,Infallible> {
        info!("Deposit request for account: {}, amount: {}", req.account, req.amount);
        let mut platform = tp.lock().unwrap();
        match platform.deposit(&req.account, req.amount) {
            Ok(_) => {
                info!("Deposit successful for account: {}, amount: {}", req.account, req.amount);
                Ok(warp::reply::json(&"Deposit successful"))
            },
            Err(e) => {
                error!("Deposit failed for account: {}, amount: {}, error: {:?}", req.account, req.amount, e);
                Ok(warp::reply::json(&format!("Error: {:?}", e)))
            },
        }
    }


    pub async fn withdraw(tp : Arc<Mutex<TradingPlatform>> , req: AccountUpdateRequest ) -> Result<impl warp::Reply ,Infallible> {
        info!("Withdraw request for account: {}, amount: {}", req.account, req.amount);
        let mut platform = tp.lock().unwrap();
        match platform.withdraw(&req.account, req.amount) {
            Ok(_) => {
                info!("Withdrawal successful for account: {}, amount: {}", req.account, req.amount);
                Ok(warp::reply::json(&"Withdrawal successful"))
            },
            Err(e) => {
                error!("Withdrawal failed for account: {}, amount: {}, error: {:?}", req.account, req.amount, e);
                Ok(warp::reply::json(&format!("Error: {:?}", e)))
            },
        }
    }

    pub async fn send(tp : Arc<Mutex<TradingPlatform>> , req: SendRequest ) -> Result<impl warp::Reply ,Infallible> {
        info!("Transfer request from: {} to: {}, amount: {}", req.sender, req.recipient, req.amount);
        let mut platform = tp.lock().unwrap();
        match platform.send(&req.sender, &req.recipient, req.amount) {
            Ok(_) => {
                info!("Transfer successful from: {} to: {}, amount: {}", req.sender, req.recipient, req.amount);
                Ok(warp::reply::json(&"Transfer successful"))
            },
            Err(e) => {
                error!("Transfer failed from: {} to: {}, amount: {}, error: {:?}", req.sender, req.recipient, req.amount, e);
                Ok(warp::reply::json(&format!("Error: {:?}", e)))
            },
        }
    }

    pub async fn order(tp : Arc<Mutex<TradingPlatform>> , req:Order ) -> Result<impl warp::Reply ,Infallible> {
        info!("Order request - signer: {}, side: {:?}, price: {}, amount: {}", req.signer, req.side, req.price, req.amount);
        let mut platform = tp.lock().unwrap();
        match platform.order(req) {
            Ok(receipt) => {
                info!("Order processed successfully - ordinal: {}, matches: {}", receipt.ordinal, receipt.matches.len());
                Ok(warp::reply::json(&receipt))
            },
            Err(e) => {
                error!("Order processing failed, error: {:?}", e);
                Ok(warp::reply::json(&format!("Error processing order: {:?}", e)))
            },
        }
    }


    //getter function for orderbook
    pub async fn orderbook(tp : Arc<Mutex<TradingPlatform>>) -> Result<impl warp::Reply, Infallible> {
        info!("Orderbook request received");
        let platform = tp.lock().unwrap();
        let orderbook = platform.orderbook();
        info!("Returning orderbook with {} orders", orderbook.len());
        Ok(warp::reply::json(&orderbook))
    }


    pub async fn balance(tp : Arc<Mutex<TradingPlatform>> , req : AccountBalanceRequest) -> Result<impl warp::Reply, Infallible> {
        info!("Balance request for account: {}", req.account);
        let mut  platform = tp.lock().unwrap();
        match platform.balance_of(&req.account) {
            Ok(balance) => {
                info!("Balance retrieved for account: {}, balance: {}", req.account, balance);
                Ok(warp::reply::json(&balance))
            },
           Err(e) => {
                error!("Balance retrieval failed for account: {}, error: {:?}", req.account, e);
                Ok(warp::reply::json(&format!("Error: {:?}", e)))
            },
        }
    }
}