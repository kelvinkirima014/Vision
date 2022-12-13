pub mod api;
pub mod order;

fn main() {
    // let api_s = api::init_kucoin_api();
    // println!("{:?}", api_s);
    let sandbox_api = api::init_sandbox_api();
    println!("{:?}", sandbox_api);
}
