use solana_client::rpc_response::Response;
use solana_sdk::signature::Signature;

pub struct MockRpcClient;

impl MockRpcClient {
    pub fn new() -> Self {
        Self
    }
    
    pub fn get_account_data(&self, _: &str) -> anyhow::Result<Vec<u8>> {
        Ok(include_bytes!("../test_programs/sample_token.so").to_vec())
    }
}