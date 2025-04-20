use reqwest::Client;
use serde_json::json;
use std::env;

#[derive(Debug)]
pub struct AlertConfig {
    pub slack_webhook: Option<String>,
    pub email_address: Option<String>,
}

pub async fn send_alert(
    config: &AlertConfig,
    program_id: &str,
    risks: &[crate::risk::Risk],
) -> anyhow::Result<()> {
    let high_risks: Vec<_> = risks.iter().filter(|r| r.level >= 4).collect();
    
    if high_risks.is_empty() {
        return Ok(());
    }

    let message = format!(
        "⚠️ *Security Alert for {}* ⚠️\n{} high-risk items found:\n{}",
        program_id,
        high_risks.len(),
        high_risks.iter()
            .map(|r| format!("- {} (Severity {})", r.description, r.level))
            .collect::<Vec<_>>()
            .join("\n")
    );

    // Send to Slack
    if let Some(webhook) = &config.slack_webhook {
        let client = Client::new();
        let payload = json!({ "text": message });
        client.post(webhook)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;
    }

    // Send to Email (via SMTP)
    if let Some(email) = &config.email_address {
        let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".into());
        let email = send_email(
            &smtp_host,
            "security-alerts@solana-inspector",
            email,
            "Security Alert",
            &message,
        ).await?;
    }

    Ok(())
}

async fn send_email(
    host: &str,
    from: &str,
    to: &str,
    subject: &str,
    body: &str,
) -> anyhow::Result<()> {
    // Implementation would use lettre or similar crate
    Ok(())
}