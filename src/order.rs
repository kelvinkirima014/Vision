use kucoin_rs::kucoin::{ user, websocket };
use std::error::Error;


pub async fn get_order_pairs() -> Result<(), Box<dyn Error>> {
     let user_info = user::get_user_subaccount_info();
    Ok(())
}