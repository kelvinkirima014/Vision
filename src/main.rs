pub mod api;

fn main() {
    let api_s = api::init_kucoin_api();
    println!("{:?}", api_s)
}
