use kucoin_rs::futures::TryStreamExt;
use kucoin_rs::tokio;
use std::error::Error;

use kucoin_rs::kucoin::client::{Kucoin, Credentials, KucoinEnv};
use kucoin_rs::kucoin::model::websocket::{KucoinWebsocketMsg, WSType, WSTopic};
use crate::api_credentials::ApiCredentials;

#[tokio::main]
pub async fn init_sandbox_api(api_credentials: ApiCredentials) -> Result<(), Box<dyn Error>> {

  
    let api_credentials = Credentials::new(&api_credentials.api_key, &api_credentials.api_secret, &api_credentials.api_pass);

    // Initialize the Kucoin API struct
    let api = Kucoin::new(KucoinEnv::Sandbox, Some(api_credentials))?;

    // Generate the dynamic Public or Private websocket url and endpoint from Kucoin
    let url = api.get_socket_endpoint(WSType::Public).await?;

    //initialize the websocket
    let mut websock = api.websocket();

    //Generate a Vec<WSTopic of desired subscriprtions. Maybe Public or Private depending on the url
    let subscriptions = vec![WSTopic::Ticker(vec![
        "BTC-USDT".to_string(),
        ])];

    //initialize the subscription and await the future
    websock.subscribe(url, subscriptions).await?;

    //Handle incoming responses with matching messages
    while let Some(msg) = websock.try_next().await? {
        match msg {
            KucoinWebsocketMsg::TickerMsg(msg) => println!("{:#?}", msg),
            KucoinWebsocketMsg::PongMsg(msg) => println!("{:#?}", msg),     // Optional
            _ => (),
        }
    }

    Ok(())
}

  