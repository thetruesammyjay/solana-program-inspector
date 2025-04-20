use crate::{InstructionMeta, AccountMeta, CpiCall};
use anyhow::{Result, Context};

const SBF_MAGIC: [u8; 4] = [0x7f, 0x53, 0x4f, 0x46]; // SBF magic number
const CPI_SIG: [u8; 4] = [0x23, 0x43, 0x50, 0x49]; // "#CPI"

pub fn parse_instructions(data: &[u8]) -> Result<Vec<InstructionMeta>> {
    let mut instructions = Vec::new();
    
    for (idx, window) in data.windows(4).enumerate() {
        if window == SBF_MAGIC {
            let start = idx + 4;
            if start + 4 > data.len() {
                continue;
            }
            
            let discriminator = hex::encode(&data[start..start+4]);
            let (name, accounts) = super::resolve_known_signature(&discriminator)
                .unwrap_or((None, Vec::new()));
            
            instructions.push(InstructionMeta {
                discriminator,
                name,
                accounts,
                data_layout: guess_data_layout(&data[start+4..]),
            });
        }
    }
    
    Ok(instructions)
}

fn guess_data_layout(data: &[u8]) -> Option<String> {
    if data.len() >= 8 {
        Some("u64 amount, u8 decimals".to_string())
    } else {
        None
    }
}

pub fn analyze_accounts(
    _data: &[u8],
    instructions: &[InstructionMeta]
) -> Result<Vec<AccountMeta>> {
    let mut accounts = Vec::new();
    
    for ix in instructions {
        for acc in &ix.accounts {
            if !accounts.iter().any(|a: &AccountMeta| a.name == acc.name) {
                accounts.push(acc.clone());
            }
        }
    }
    
    Ok(accounts)
}

pub fn find_cross_program_calls(data: &[u8]) -> Result<Vec<CpiCall>> {
    let mut cpis = Vec::new();
    
    for (idx, window) in data.windows(4).enumerate() {
        if window == CPI_SIG {
            let start = idx + 4;
            if start + 32 > data.len() {
                continue;
            }
            
            let program_id = bs58::encode(&data[start..start+32]).into_string();
            cpis.push(CpiCall {
                program_id,
                instruction: "unknown".to_string(),
                accounts: Vec::new(),
            });
        }
    }
    
    Ok(cpis)
}