use crate::{InstructionMeta, AccountMeta, CpiCall};
use anyhow::Result;
use std::collections::HashMap;

pub mod sbf;

lazy_static::lazy_static! {
    static ref KNOWN_SIGNATURES: HashMap<&'static str, (&'static str, Vec<AccountMeta>)> = {
        let mut m = HashMap::new();
        m.insert("12a34b56", (
            "initialize",
            vec![
                AccountMeta {
                    name: "admin".into(),
                    is_signer: true,
                    is_writable: true,
                    is_pda: false,
                }
            ]
        ));
        m
    };
}

pub fn extract_instructions(data: &[u8]) -> Result<Vec<InstructionMeta>> {
    sbf::parse_instructions(data)
}

pub fn infer_account_layouts(
    data: &[u8],
    instructions: &[InstructionMeta]
) -> Result<Vec<AccountMeta>> {
    sbf::analyze_accounts(data, instructions)
}

pub fn find_cpi_signatures(data: &[u8]) -> Result<Vec<CpiCall>> {
    sbf::find_cross_program_calls(data)
}

pub fn resolve_known_signature(discriminator: &str) -> Option<(&'static str, Vec<AccountMeta>)> {
    KNOWN_SIGNATURES.get(discriminator).copied()
}