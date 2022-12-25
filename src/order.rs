use kucoin_rs::kucoin::{ user, websocket };
use kucoin_rs::kucoin::client::Kucoin;
use std::error::Error;


pub async fn get_order_pairs(user_i: Kucoin) -> Result<(), Box<dyn Error>> {
    
     let user_info = user_i.get_user_subaccount_info();
    Ok(())
}