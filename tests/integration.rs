use solana_program_inspector::{analyze_binary, analyze_program};
use anyhow::Result;

#[test]
fn test_token_program_analysis() -> Result<()> {
    let data = include_bytes!("../test_programs/sample_token.so");
    let analysis = analyze_binary(data)?;
    assert!(!analysis.instructions.is_empty());
    Ok(())
}

#[test]
fn test_upgradeable_program_detection() -> Result<()> {
    let data = include_bytes!("../test_programs/sample_upgradeable.so");
    let analysis = analyze_binary(data)?;
    assert!(analysis.risks.iter().any(|r| r.category == "Upgradability"));
    Ok(())
}