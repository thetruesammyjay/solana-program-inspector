use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

pub fn detect_program_version(
    rpc_url: &str,
    program_id: &Pubkey,
) -> anyhow::Result<Option<String>> {
    let client = RpcClient::new(rpc_url);
    let account = client.get_account(program_id)?;
    
    // Extract version from program data (heuristic)
    let data = account.data;
    if data.len() > 100 {
        // Check for common version patterns
        let version_sigs = [
            (b"version=", 8),
            (b"solana-program-", 14),
        ];
        
        for (sig, offset) in version_sigs {
            if let Some(pos) = data.windows(sig.len()).position(|w| w == sig) {
                let start = pos + offset;
                let end = data[start..].iter()
                    .position(|&b| b == b'\0' || b == b'\n')
                    .unwrap_or(10);
                return Ok(Some(String::from_utf8_lossy(&data[start..start+end]).into()));
            }
        }
    }
    
    Ok(None)
}