# 🔒 WALEED-CORE | Systems Security Engine

<p align="left">
  <img src="https://img.shields.io/badge/Architect-Eng--MohamedWaleed1-blue?style=for-the-badge">
  <img src="https://img.shields.io/badge/Security-Level%204-red?style=for-the-badge">
  <img src="https://img.shields.io/badge/Memory-Zero--Copy-green?style=for-the-badge">
  <img src="https://img.shields.io/badge/Status-Final--Stable-gold?style=for-the-badge">
</p>

---

## 🚀 System Overview
A high-performance backend engine focused on secure data transformation. **WALEED-CORE** implements advanced memory management protocols to ensure immunity against common system vulnerabilities (buffer overflows, data races) — enforced at **compile time** via Rust's rigorous safety model.

---

## 📖 User Manual & Deployment
This project is deployed as two independent security tools. You can download the ready-to-use binaries from the **[Releases]** section on the right.

### **Phase 1: Encryption (The Sender)**
1. Run `sender.exe`.
2. Enter your secret text.
3. The system generates a **Hexadecimal Cipher**. Copy it.
4. Press **ENTER** to wipe memory and exit.

### **Phase 2: Decryption (The Receiver)**
1. Run `receiver.exe`.
2. Paste the **Hexadecimal Cipher**.
3. The system validates integrity and displays the **Original Message**.
4. Press **ENTER** to safely clear the buffer.

---

## 🛠 Technical Specifications
| Property | Implementation |
|---|---|
| **Memory Safety** | Enforced via Rust's Ownership & Affine type system |
| **Encryption** | Positional XOR stream cipher with bit-rotation |
| **Integrity Checks** | FNV-1a 64-bit checksum verification |
| **Secure Erasure** | RAII-based zeroing via `Drop` trait (Zero-Fill) |
| **Architecture** | Zero-cost abstractions, Zero heap copies |

---

## 📐 Core Design Principles
### 1. Ownership-Driven Lifecycle
The system ensures that plaintext ownership is **moved** into the secure buffer, rendering the original data binding invalid to prevent leaks.

### 2. Lifetime-Bounded Views
Using Rust lifetimes (`<'buf>`), we guarantee that any read-only view of the sensitive data cannot outlive the secure container.

### 3. Deterministic State Machine
`Idle ──load()──► Loaded ──encrypt()──► Encrypted ──decrypt()──► Decrypted`

---

## 📁 Repository Structure
security-core/
├── Cargo.toml                # Package manifest
├── README.md                 # System Overview & Manual
├── SECURITY.md               # Privacy & Data Handling Policy
├── ENGINEERING_ANALYSIS.md   # Full technical design report
└── src/
├── sender.rs             # Encryption frontend
├── receiver.rs           # Decryption frontend
└── engine.rs             # Core: SecureBuffer & Crypto Logic


---
*© 2026 Eng-MohamedWaleed1 | European University of Lefke — Faculty of Engineering*
