# Solana Program Inspector 🔍
**Reverse engineering toolkit for closed-source Solana programs**

## 📌 Overview

The Solana Program Inspector is a security-focused toolkit that:
- **Lifts the veil** on closed-source Solana programs
- **Automatically extracts** instructions, account layouts, and CPIs
- **Detects risks** like upgradeable programs and freeze authorities
- **Generates audit-ready reports** in JSON/Markdown formats

Built for security researchers, auditors, and developers navigating Solana's opaque program ecosystem.

---

## 🚀 Features

### Core Analysis
- Instruction discriminator extraction
- Account structure reconstruction
- Cross-program invocation mapping
- Data layout inference
- Program version detection
- Solana advisory database integration

### Security Focus
- Risk scoring (1-5 severity)
- Upgradeability detection
- Admin function identification
- Freeze authority checks
- Real-time Slack/email alerts
- CVE pattern matching

### Output Formats
```json
{
  "risks": [{
    "level": 5,
    "category": "Upgradability",
    "description": "Program can be upgraded",
    "mitigation": "Verify multisig control",
    "advisory": "SOL-2023-1"
  }]
  "version": "1.16.0"
}
```

---

## 🛠 Installation

### Windows Setup
1. **Install Rust** (if not installed):
    ```powershell
    winget install Rustlang.Rustup
    ```
2. **Clone repository:**
    ```cmd
    git clone https://github.com/thetruesammyjay/solana-program-inspector.git
    cd solana-program-inspector
    ```
3. **Build project:**
    ```cmd
    cargo build --release
    ```

### Linux/macOS
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --git https://github.com/thetruesammyjay/solana-program-inspector
```

### With Alerting Support
```bash
cargo build --release --features alerting
```

### Environment Variables for Alerts
```bash
export SLACK_WEBHOOK="https://hooks.slack.com/services/..."
export ALERT_EMAIL="security@yourdomain.com"
export SMTP_HOST="smtp.gmail.com"
```
---

## 💻 Usage
- **Analyze On-Chain Program**
```bash
cargo run -- analyze --program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
```

- **Analyze Local Binary**
```bash
cargo run -- analyze-binary ./mystery_program.so --output report.json
```

- **Detect Version + Analyze**
```bash
cargo run -- analyze --program <ID> --check-version
```

- **With Alerting**
```bash
cargo run --features alerting -- analyze --program <ID> --alert
```

---

## 🧰 Advanced Features
- **Ghidra Integration**
```python
# scripts/ghidra_plugin.py
analyze_solana_program()  # Auto-detects SBF patterns
```
- **Custom Risk Patterns**
Add to `src/risk/patterns/upgradeable_programs`:
```text
# New pattern example
change_admin:accounts=owner(S),new_admin(W)
```

---

## 📊 Comparison to Alternatives

| Feature               | Our Tool | Solscan | Anchor Verify |
|-----------------------|:--------:|:-------:|:-------------:|
| Instruction Recovery  | ✅       | ❌      | Partial       |
| Account Inference     | ✅       | ❌      | ❌            |
| Risk Detection        | ✅       | ❌      | ❌            |
| Local Binary Support  | ✅       | ❌      | ❌            |

---

## 🛠 Troubleshooting (Windows)
### 1. Rust/Cargo Not Recognized
**Symptoms**:
```cmd
'cargo' is not recognized as an internal or external command
```
**Fix**:
```cmd
setx PATH "%PATH%;%USERPROFILE%\.cargo\bin"
```
Then **restart your terminal.**

### 2. Linker Errors (LNK1181)
**Symptoms**:
```log
error: linker `link.exe` not found
```
**Solution**:
**Step 1**: Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
**Step 2**: Select:
    - "Desktop development with C++"
    - "Windows 10/11 SDK" 

### 3. Program Binary Analysis Fails
**Symptoms**:
```log
Error: Invalid SBF binary (too short)
```
**Fix**:
```powershell
# Ensure valid Solana program
solana program dump <PROGRAM_ID> program.so
cargo run -- analyze-binary program.so
```
### 4. Slow Performance
**Optimizations**:
```cmd
:: Build in release mode
cargo build --release

:: Limit RPC requests
cargo run -- analyze --program <ID> --rpc <PRIVATE_RPC_URL>
```
### 5. Ghidra Plugin Issues
**Debugging Steps**:
    - 1. Make sure Ghidra is added to PATH:
    ```cmd
    setx PATH "%PATH%;C:\ghidra_<version>"
    ```
    - 2. Run analysis manually:
    ```powershell
    .\ghidraRun .\scripts\ghidra_plugin.py
    ```
### 6. Permission Denied Errors
**Solution**:
```powershell
# Run as admin (if needed)
Start-Process cmd -Verb RunAs -ArgumentList "/k cd $PWD"
```
### 7. Outdated Dependencies
**Update Check**:
```cmd
cargo update
rustup update
solana-install update
```

---

## 🤝 Contributing
1. Fork the repository

2. Create a feature branch (`git checkout -b feature/amazing`)

3. Commit changes (`git commit -m 'Add amazing feature'`)

4. Push to branch (`git push origin feature/amazing`)

5. Open a Pull Request

---

## 📬 Contact

**Security Researchers & Collaborators Welcome!**

- **X (Twitter):** [@thatbwoysammyj](https://x.com/thatbwoysammyj)  
- **Telegram:** [t.me/sammyjayisthename](https://t.me/sammyjayisthename)  
- **Email:** [thetruesammyjay@gmail.com](mailto:thetruesammyjay@gmail.com)

---

## 📜 License

**MIT License** – See [LICENSE](https://license.md/) for details
