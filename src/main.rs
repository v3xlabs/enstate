use ethers::prelude::*;

mod database;
mod http;
mod routes;
mod state;
mod oapi;
mod abi;

use state::AppState;

#[tokio::main]
async fn main() {
    println!("enstate.rs v{}", env!("CARGO_PKG_VERSION"));

    database::setup().await;

    let client = Provider::<Http>::try_from("https://rpc.ankr.com/eth").unwrap();

    let state = AppState {
        provider: client,
    };

    let router = http::setup(state);
    
    http::start(router).await;
}





// let contract = MyThingssssss::new(H160::from_str("0x57f1887a8BF19b14fC0dF6Fd9B2acc9Af147eA85").unwrap(), Arc::new(client));
// let v = contract.balance_of(H160::from_str("0x225f137127d9067788314bc7fcc1f36746a3c3B5").unwrap()).await.unwrap();
// println!("balance: {}", v);
