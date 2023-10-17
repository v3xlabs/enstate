use worker::*;

use ethers::providers::{Http, Middleware, Provider};

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let rpc = Provider::<Http>::try_from(
        "https://rpc.ankr.com/eth/17c7bd60d262bc06008f0a111ca740955ee09d4bb33ecb57d2700214c8f625f1",
    )
    .unwrap();

    let block = rpc.get_block_number().await.unwrap();

    Response::ok(format!(
        "Hello from Rust! The current block number is {}",
        block
    ))
}
