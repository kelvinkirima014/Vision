pub mod api;
pub mod order;
pub mod api_credentials;

fn main() {
    let sandbox_creds = api_credentials::load_api_credentials();
    let sandbox_api = api::init_sandbox_api(sandbox_creds);
    println!("{:?}", sandbox_api);
}
