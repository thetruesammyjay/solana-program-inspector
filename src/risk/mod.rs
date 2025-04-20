use crate::{
    advisory::{self, SecurityAdvisory},
    InstructionMeta, AccountMeta, Risk,
};
use solana_program::pubkey::Pubkey;
use std::collections::HashSet;

/// Risk analysis with Solana security advisory integration
pub fn analyze_risks(
    program_id: &str,
    instructions: &[InstructionMeta],
    accounts: &[AccountMeta],
) -> Vec<Risk> {
    let mut risks = Vec::new();

    // 1. Check Solana Security Advisories
    risks.extend(check_security_advisories(program_id));

    // 2. Detect upgradeable programs
    risks.extend(detect_upgradeability(instructions));

    // 3. Identify dangerous account patterns
    risks.extend(analyze_account_risks(accounts));

    // 4. Instruction-specific risks
    risks.extend(analyze_instruction_risks(instructions));

    risks
}

/// Check against Solana's security advisory database
fn check_security_advisories(program_id: &str) -> Vec<Risk> {
    advisory::check_advisories(program_id)
        .into_iter()
        .map(|adv| Risk {
            level: adv.severity,
            category: "Security Advisory".to_string(),
            description: format!("{}: {}", adv.id, adv.title),
            mitigation: adv.patched_versions.as_ref().map_or(
                "No official patch available".to_string(),
                |v| format!("Upgrade to version {} or later", v.join(", ")),
            ),
            references: adv.references.clone(),
        })
        .collect()
}

/// Detect program upgrade risks
fn detect_upgradeability(instructions: &[InstructionMeta]) -> Vec<Risk> {
    let upgrade_indicators = &["upgrade", "set_admin", "change_authority"];
    
    instructions
        .iter()
        .filter(|ix| {
            ix.name.as_ref().map_or(false, |name| {
                upgrade_indicators.iter().any(|kw| name.contains(kw))
            })
        })
        .map(|ix| Risk {
            level: 5,
            category: "Upgradability".to_string(),
            description: format!(
                "Program can be upgraded via '{}' instruction",
                ix.name.as_deref().unwrap_or("unknown")
            ),
            mitigation: Some(
                "Require multisig/timelock for upgrades".to_string(),
            ),
            references: vec![],
        })
        .collect()
}

/// Analyze account-level risks
fn analyze_account_risks(accounts: &[AccountMeta]) -> Vec<Risk> {
    let mut risks = Vec::new();
    let privileged_accounts: HashSet<_> = ["admin", "authority", "owner", "freeze"].into_iter().collect();

    for acc in accounts {
        if privileged_accounts.contains(acc.name.as_str()) {
            let risk_level = match (acc.is_signer, acc.is_writable) {
                (true, true) => 5,  // Signer + Writable
                (true, false) => 4, // Signer only
                (false, true) => 3, // Writable only
                _ => 1,
            };

            risks.push(Risk {
                level: risk_level,
                category: "Privileged Account".to_string(),
                description: format!(
                    "Sensitive account '{}' (signer: {}, writable: {})",
                    acc.name, acc.is_signer, acc.is_writable
                ),
                mitigation: Some(
                    "Implement multi-sig protection".to_string(),
                ),
                references: vec![],
            });
        }
    }

    risks
}

/// Instruction-specific risk patterns
fn analyze_instruction_risks(instructions: &[InstructionMeta]) -> Vec<Risk> {
    let mut risks = Vec::new();
    let dangerous_ops = &[
        ("freeze", 5),
        ("thaw", 4),
        ("mint_to", 4),
        ("burn", 3),
    ];

    for (op, severity) in dangerous_ops {
        if instructions.iter().any(|ix| {
            ix.name.as_ref().map_or(false, |name| name.contains(op))
        }) {
            risks.push(Risk {
                level: *severity,
                category: "Dangerous Operation".to_string(),
                description: format!("'{}' instruction detected", op),
                mitigation: Some(
                    "Add proper access controls".to_string(),
                ),
                references: vec![],
            });
        }
    }

    risks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advisory::SecurityAdvisory;

    #[test]
    fn test_advisory_check() {
        let test_adv = SecurityAdvisory {
            id: "TEST-2023-1".to_string(),
            title: "Test Advisory".to_string(),
            affected_programs: vec!["TESTPROGRAM".to_string()],
            severity: 4,
            patched_versions: Some(vec!["1.0.1".to_string()]),
            references: vec![],
        };

        let risks = check_security_advisories("TESTPROGRAM");
        assert_eq!(risks[0].level, 4);
        assert!(risks[0].description.contains("TEST-2023-1"));
    }
}