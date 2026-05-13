# 🔒 WALEED-CORE | Systems Security Engine

<p align="left">
  <img src="https://img.shields.io/badge/Architect-Eng--MohamedWaleed1-blue?style=for-the-badge">
  <img src="https://img.shields.io/badge/Security-Level%204-red?style=for-the-badge">
  <img src="https://img.shields.io/badge/Memory-Zero--Copy-green?style=for-the-badge">
  <img src="https://img.shields.io/badge/Dependencies-None-orange?style=for-the-badge">
</p>

---

## 🚀 System Overview

A high-performance backend engine focused on secure data transformation. **WALEED-CORE** implements advanced memory management protocols to ensure immunity against common system vulnerabilities including use-after-free, buffer overflows, and data races — enforced at **compile time**, not runtime.

---

## 🛠 Technical Specifications

| Property | Implementation |
|---|---|
| **Memory Safety** | Enforced via Rust's Ownership model (affine type system) |
| **Encryption** | Positional XOR stream cipher with bit-rotation diffusion |
| **Integrity** | FNV-1a 64-bit checksum, verified on every decryption |
| **Secure Erasure** | RAII-based zeroing via `Drop` trait — runs even on panic |
| **Architecture** | Zero-cost abstractions, zero heap copies in hot path |
| **Dependencies** | None — Rust Standard Library only |
| **Test Coverage** | 16 tests across state, crypto, borrow, and edge-case categories |

---

## 📐 Core Design Principles

### 1. Ownership-Driven Lifecycle
```
load(plaintext: Vec<u8>)
     │
     └── Ownership MOVED into buffer
         Caller's binding: INVALID
         Heap copies: ZERO
```

### 2. Lifetime-Bounded Immutable Views
```rust
let view: BufferView<'buf> = buffer.view();
// 'buf guarantees view cannot outlive buffer
// buffer is immutable for the entire view lifetime
```

### 3. Deterministic State Machine
```
Idle ──load()──► Loaded ──encrypt()──► Encrypted ──decrypt()──► Decrypted
  ▲                                                                    │
  └──────────────────────── clear() ──────────────────────────────────┘
```

---

## 📦 Deployment

```bash
# Compile and run the engine boot sequence
cargo run

# Run the full test suite (16 tests)
cargo test

# Run with output visible
cargo test -- --nocapture
```

---

## 📁 Repository Structure

```
waleed-security-core/
├── Project.toml              # Package manifest
├── README.md                 # This file
├── ENGINEERING_ANALYSIS.md   # Full technical design report
└── src/
    ├── main.rs               # Engine boot sequence and demo
    └── engine.rs             # Core: SecureBuffer, cipher, integrity
```

---

## 📄 Documentation

Full engineering analysis — including cipher design rationale, state machine specification, FNV-1a integrity layer, and production hardening roadmap — is provided in [`ENGINEERING_ANALYSIS.md`](./ENGINEERING_ANALYSIS.md).

---

*© 2026 Eng-MohamedWaleed1 | Software Engineering Portfolio*
*European University of Lefke — Faculty of Engineering*
