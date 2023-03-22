//use kucoin_rs::kucoin::{ user, websocket };
use kucoin_rs::kucoin::client::Kucoin;
use std::error::Error;


pub async fn order(user_i: Kucoin) -> Result<(), Box<dyn Error>> {

    let account = user_i.get_account(account_id);
    
     let _user_info = user_i.get_user_subaccount_info();
    Ok(())
}

