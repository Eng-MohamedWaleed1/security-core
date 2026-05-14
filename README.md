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

## ⚡ Quick Start — No Installation Required

> Download the ready-to-use `.exe` files from the **[Releases](../../releases)** section on the right side of this page.

### Step 1 — Sender (You)
1. Download and run **`sender.exe`**
2. If Windows shows a security warning → click **"More info"** then **"Run anyway"**
3. Type your secret message and press **ENTER**
4. Copy the **Hexadecimal Cipher** that appears
5. Send it to the receiver via WhatsApp, Telegram, etc.

### Step 2 — Receiver (Your contact)
1. Download and run **`receiver.exe`**
2. If Windows shows a security warning → click **"More info"** then **"Run anyway"**
3. Paste the **Hexadecimal Cipher** and press **ENTER**
4. The original message appears instantly
5. Press **ENTER** to wipe memory and exit

---

## 🔨 Build From Source (Developers Only)

> Requires [Rust](https://rustup.rs) installed on your machine.

```bash
git clone https://github.com/Eng-MohamedWaleed1/waleed-security-core.git
cd waleed-security-core
cargo build --release
cargo run --bin sender
cargo run --bin receiver
```

Compiled binaries appear at `target/release/sender.exe` and `target/release/receiver.exe`.

---

## 🛠 Technical Specifications

| Property | Implementation |
|---|---|
| **Memory Safety** | Enforced via Rust's Ownership & Affine type system |
| **Encryption** | Positional XOR stream cipher with bit-rotation diffusion |
| **Integrity Checks** | FNV-1a 64-bit checksum verification on every decryption |
| **Secure Erasure** | RAII-based zeroing via `Drop` trait (Zero-Fill on scope exit) |
| **Architecture** | Zero-cost abstractions, zero heap copies in hot path |
| **Dependencies** | None — Rust Standard Library only |

---

## 📐 Core Design Principles

### 1. Ownership-Driven Lifecycle
Plaintext ownership is **moved** into the secure buffer — the original binding is invalidated immediately, preventing accidental leaks.

### 2. Lifetime-Bounded Immutable Views
Using Rust lifetimes (`<'buf>`), any read-only view of sensitive data is guaranteed by the compiler to not outlive its secure container.

### 3. Deterministic State Machine

```
Idle ──load()──► Loaded ──encrypt()──► Encrypted ──decrypt()──► Decrypted
  ▲                                                                    │
  └──────────────────────── clear() / Drop ───────────────────────────┘
```

---

## 📁 Repository Structure

```
waleed-security-core/
├── Cargo.toml
├── README.md
├── ENGINEERING_ANALYSIS.md
└── src/
    ├── engine.rs
    ├── sender.rs
    └── receiver.rs
```

---

## ⚠️ Security Notice

The `.exe` files may trigger a Windows SmartScreen warning because they are not commercially code-signed. This is expected for open-source tools. The full source code is available above for independent verification.

---

*© 2026 Eng-MohamedWaleed1 | European University of Lefke — Faculty of Engineering*
