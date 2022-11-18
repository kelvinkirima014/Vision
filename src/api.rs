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

    // Initialize the Kucoin API struct
    let api = Kucoin::new(KucoinEnv::Live, Some(credentials))?;
     
    // Generate the dynamic Public or Private websocket url and endpoint from Kucoin
    // which includes a token required for connecting
    let url = api.get_socket_endpoint(WSType::Public).await?;
     
    // Initialize the websocket
    let mut ws = api.websocket();

    // Generate a Vec<WSTopic> of desired subs.They need to be public or private depending on the url
   // let subs = vec![WSTopic::Ticker(vec!["BTC-USDT".to_string()])];
   let subs = vec![WSTopic::AllTicker];
     
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

