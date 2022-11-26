use kucoin_rs::futures::TryStreamExt;
use kucoin_rs::tokio;
use kucoin_rs::failure;
use kucoin_rs::kucoin::client::{Kucoin, Credentials, KucoinEnv};
use kucoin_rs::kucoin::model::websocket::{KucoinWebsocketMsg, WSType, WSTopic};
use dotenv::dotenv;
use std::env;

#[tokio::main]
pub async fn api_helpers() -> Result<(), failure::Error>  {
    dotenv().ok();
    
    let api_key = env::var("KUCOIN_API_KEY").expect("missing API key");
    let secret_key = env::var("KUCOIN_SECRET_KEY").expect("wrong secret key");
    let passphrase = env::var("KUCOIN_PASSPHRASE").expect("Wrong passphrase");
    //generate a new Credentials struct w/ the necessary keys
    let credentials = Credentials::new(
        &api_key,
        &secret_key,
        &passphrase,
    );

   // let credentials = get_credentials(Credentials::new(&API_KEY, &SECRET_KEY, &PASSPHRASE));

    // Initialize the Kucoin API struct
    let api = Kucoin::new(KucoinEnv::Live, Some(credentials))?;
     
    // Generate the dynamic Public or Private websocket url and endpoint from Kucoin
    // which includes a token required for connecting
    let url = api.get_socket_endpoint(WSType::Public).await?;
     
    // Initialize the websocket
    let mut ws = api.websocket();

    // Generate a Vec<WSTopic> of desired subs.They need to be public or private depending on the url
    let subs = vec![WSTopic::Ticker(vec!["BTC-USDT".to_string()])];
    // let subs = vec![WSTopic::AllTicker];
     
    // Initalize your subscription and use await to unwrap the future   
    ws.subscribe(url, subs).await?;    
    // Handle incoming responses matching messages
    while let Some(msg) = ws.try_next().await? {
        match msg {
            KucoinWebsocketMsg::TickerMsg(msg) => println!("{:#?}", msg),
            KucoinWebsocketMsg::PongMsg(msg) => println!("{:#?}", msg),     // Optional
            KucoinWebsocketMsg::WelcomeMsg(msg) => println!("{:#?}", msg),  // Optional
            _ => (),
        }
    }
    Ok(())
}

// pub async fn place_market_order(
//     api: &ftx::rest::Rest,
//     market_name: &str,
//     order_side: ftx::rest::Side,
//     order_size: rust_decimal::Decimal) -> bool {
//     let order = api.request(ftx::rest::PlaceOrder {
//         market: std::string::String::from(market_name),
//         side: order_side,
//         price: None,
//         r#type: ftx::rest::OrderType::Market,
//         size: order_size,
//         reduce_only: false,
//         ioc: false,
//         post_only: false,
//         client_id: None,
//         reject_on_price_band: false,
//     }).await;

//     let order_success;
//     match order {
//         Err(e) => {
//             log::error!("Unable to place order, Err: {:?}", e);
//             order_success = false;
//         }
//         Ok(o) => {
//             log::info!("Order placed successfully: {:?}", o);
//             order_success = true;
//         }
//     };

//     return order_success;
// }

// async fn get_account_info(info: String) -> Result<()> {

//     Ok(())
// }

// pub struct TradingPair {
//     symbol: String,
//     base_asset: String,
//     quote_asset: String,
//     step: f64,
//     initialized: bool,
//     asks: Vec<Asks>,
//     bids: Vec<Bids>,
//     timestamp: SystemTime,
//   }
  
//   impl fmt::Display for TradingPair {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//       write!(f, "<TradingPair {}/{}>", self.base_asset, self.quote_asset)
//     }
//   }
  
//   impl TradingPair {
//     // Constructor
//     pub fn new(symbol: String, base_asset: String, quote_asset: String, step: f64) -> TradingPair {
//       TradingPair {
//         symbol,
//         base_asset,
//         quote_asset,
//         step,
//         initialized: false,
//         asks: Vec::new(),
//         bids: Vec::new(),
//         timestamp: SystemTime::now(),
//       }
//     }
//     // Setters
//     pub fn update(&mut self, timestamp: SystemTime, asks: Vec<Asks>, bids: Vec<Bids>) {
//       self.asks = asks;
//       self.bids = bids;
//       self.timestamp = timestamp;
//       self.initialized = true;
//     }
//     // Getters
//     pub fn get_symbol(&self) -> String {
//       self.symbol.to_string()
//     }
//     pub fn get_step(&self) -> f64 {
//       self.step
//     }
//     pub fn get_base_asset(&self) -> String {
//       self.base_asset.to_string()
//     }
//     pub fn get_quote_asset(&self) -> String {
//       self.quote_asset.to_string()
//     }
//     // Utilities
//     pub fn has_asset(&self, asset: String) -> bool {
//       if (asset == self.quote_asset) || (asset == self.base_asset) {
//         return true;
//       }
//       false
//     }
//     pub fn get_the_other(&self, asset: String) -> String {
//       if asset == self.quote_asset {
//         self.base_asset.to_string()
//       } else if asset == self.base_asset {
//         self.quote_asset.to_string()
//       } else {
//         "".to_string()
//       }
//     }
  
//     pub fn text(&self) -> String {
//       format!("{}/{}", self.base_asset, self.quote_asset)
//     }
//   }
  