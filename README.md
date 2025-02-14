# 🚀 UpOnly - Solana-Based Token Launchpad

UpOnly is a **decentralized token launchpad** built on **Solana**, featuring **governance, staking, trading, and anti-rug mechanisms** to create a **secure and transparent** crypto ecosystem.

## 🌟 Key Features
✅ **Token Creation** – Launch SPL tokens with minting & freezing authority.  
✅ **Governance Staking** – Stake tokens for voting power & rewards.  
✅ **Decentralized Trading** – Securely buy & sell tokens within the ecosystem.  
✅ **Anti-Rug Mechanisms** – Lock selling via governance to prevent mass sell-offs.  
✅ **Engagement-Based Rewards** – Incentivize long-term holding & community participation.  

---

## 📂 Project Structure
 
---

## ⚡ How It Works

### 1️⃣ **Token Creation**
- Create new **SPL tokens** on Solana.
- Assign **mint & freeze authority** for security.
- **Mint new tokens** to specific accounts.

### 2️⃣ **Staking**
- Stake tokens in the **staking pool** for rewards.
- Governance power **increases with stake amount**.
- **Unstaking locked for 7 days** to prevent quick dumps.

### 3️⃣ **Decentralized Trading**
- **Buy & sell tokens** securely on-chain.
- Trading **automatically updates total volume**.
- **Governance controls sell restrictions**.

### 4️⃣ **Governance & Voting**
- **Token holders vote on key decisions** (e.g., locking token sales).
- Majority vote (60%+) **can lock or unlock token sales**.
- Ensures **fair & decentralized decision-making**.

### 5️⃣ **Selling Lock Mechanism**
- **Governance can lock selling** to prevent market dumps.
- Requires **majority vote (60%)** for activation.
- **Locks sell transactions for a set duration**.

---

## 🔧 Installation & Deployment

### **1️⃣ Install Dependencies**
Ensure you have **Rust, Solana CLI, and Anchor** installed.

```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
anchor build

anchor test

solana config set --url devnet
anchor deploy



---

### **How to Use This README**
1. Replace placeholders (`[your-email@example.com]`, `[your_project]`, `[your-website.com]`, etc.) with your actual project details.
2. Add **links to relevant resources** such as documentation, whitepapers, or user guides.
3. Upload this file to **GitHub as `README.md`** in your repo’s root directory.

This README **clearly explains the project, its features, how to install it, and how to contribute** while maintaining a **professional and structured format**. 🚀🔥




# Install Rust & Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Anchor Framework
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
