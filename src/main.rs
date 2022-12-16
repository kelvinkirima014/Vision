pub mod api;
pub mod order;
pub mod api_credentials;

fn main() {
    let creds = api_credentials::load_api_credentials();
    let api_s = api::init_kucoin_api(creds);
    println!("{:?}", api_s);

    let sandbox_creds = api_credentials::load_api_credentials();
    let sandbox_api = api::init_sandbox_api(sandbox_creds);
    println!("{:?}", sandbox_api);
}
