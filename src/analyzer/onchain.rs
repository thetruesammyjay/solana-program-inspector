use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use anyhow::{Result, Context};

pub fn fetch_recent_transactions(
    rpc_url: &str,
    program_id: &str,
    limit: usize,
) -> Result<Vec<Signature>> {
    let client = RpcClient::new_with_commitment(
        rpc_url,
        CommitmentConfig::confirmed()
    );
    
    client.get_signatures_for_address(
        &program_id.parse()?,
        None,
        Some(limit)
    ).context("Failed to fetch transactions")
}