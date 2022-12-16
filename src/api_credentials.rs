use kucoin_rs::{serde_json, serde_derive::{Serialize, Deserialize}};

static  CREDENTIALS_JSON: &'static str = include_str!("./credentials.json");

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiCredentials {
    pub(crate) name: String,
    pub(crate) api_key: String,
    pub(crate) api_secret: String,
    pub(crate) api_pass: String,
}

pub fn load_api_credentials() -> ApiCredentials {
    let mut deserialize_creds = serde_json::from_str::<Vec<ApiCredentials>>(&CREDENTIALS_JSON).expect("Error deserializing JSON data");
    deserialize_creds.swap_remove(0)
    
}