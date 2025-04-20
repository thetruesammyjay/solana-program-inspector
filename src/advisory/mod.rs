use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityAdvisory {
    pub id: String,
    pub title: String,
    pub affected_programs: Vec<String>,
    pub severity: u8, // 1-5
    pub patched_versions: Option<Vec<String>>,
    pub references: Vec<String>,
}

lazy_static::lazy_static! {
    pub static ref ADVISORY_DB: HashMap<String, SecurityAdvisory> = {
        let mut m = HashMap::new();
        m.insert(
            "SOL-2023-1".to_string(),
            SecurityAdvisory {
                id: "SOL-2023-1".to_string(),
                title: "Token Program Freeze Vulnerability".to_string(),
                affected_programs: vec![
                    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string()
                ],
                severity: 4,
                patched_versions: Some(vec!["1.16.0".to_string()]),
                references: vec![
                    "https://github.com/solana-foundation/security-advisories/blob/master/SOL-2023-1.md".to_string()
                ],
            }
        );
        m
    };
}

pub fn check_advisories(program_id: &str) -> Vec<&'static SecurityAdvisory> {
    ADVISORY_DB.values()
        .filter(|adv| adv.affected_programs.contains(&program_id.to_string()))
        .collect()
}