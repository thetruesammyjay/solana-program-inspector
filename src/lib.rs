pub mod analyzer;
pub mod extractor;
pub mod risk;

use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisOutput {
    pub program_id: Option<String>,
    pub instructions: Vec<InstructionMeta>,
    pub accounts: Vec<AccountMeta>,
    pub cpis: Vec<CpiCall>,
    pub risks: Vec<Risk>,
    pub stats: AnalysisStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstructionMeta {
    pub discriminator: String,
    pub name: Option<String>,
    pub accounts: Vec<AccountMeta>,
    pub data_layout: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountMeta {
    pub name: String,
    pub is_signer: bool,
    pub is_writable: bool,
    pub is_pda: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpiCall {
    pub program_id: String,
    pub instruction: String,
    pub accounts: Vec<AccountMeta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Risk {
    pub level: u8, // 1-5
    pub category: String,
    pub description: String,
    pub mitigation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisStats {
    pub instruction_count: usize,
    pub risky_instructions: usize,
    pub unknown_instructions: usize,
}

pub fn analyze_program(program_id: &str, rpc_url: &str) -> Result<AnalysisOutput> {
    let program_data = analyzer::fetch_program_data(rpc_url, program_id)
        .context("Failed to fetch program data")?;
    
    analyze_binary(&program_data)
        .map(|mut output| {
            output.program_id = Some(program_id.to_string());
            output
        })
}

pub fn analyze_binary(data: &[u8]) -> Result<AnalysisOutput> {
    anyhow::ensure!(!data.is_empty(), "Empty program data");
    anyhow::ensure!(data.len() > 8, "Invalid SBF binary (too short)");

    let instructions = extractor::extract_instructions(data)
        .context("Instruction extraction failed")?;
    
    let accounts = extractor::infer_account_layouts(data, &instructions)
        .context("Account analysis failed")?;
    
    let cpis = extractor::find_cpi_signatures(data)
        .context("CPI detection failed")?;
    
    let risks = risk::analyze_risks(&instructions, &accounts);
    
    let stats = AnalysisStats {
        instruction_count: instructions.len(),
        risky_instructions: risks.iter().filter(|r| r.level >= 3).count(),
        unknown_instructions: instructions.iter().filter(|i| i.name.is_none()).count(),
    };

    Ok(AnalysisOutput {
        program_id: None,
        instructions,
        accounts,
        cpis,
        risks,
        stats,
    })
}