use solana_client::rpc_client::RpcClient;
use anyhow::{Result, Context};

pub mod onchain;
pub mod benchmarks;

pub fn fetch_program_data(rpc_url: &str, program_id: &str) -> Result<Vec<u8>> {
    let client = RpcClient::new(rpc_url);
    client.get_account_data(&program_id.parse()?)
        .context("Failed to fetch program data from RPC")
}