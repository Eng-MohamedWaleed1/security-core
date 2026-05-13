# Security & Privacy Policy

## 1. Data Handling
Waleed Security Core operates under a **No-Persistence** policy. We do not write messages, keys, or logs to the hard drive. All processing occurs strictly within the volatile memory (RAM).

## 2. Memory Scrubbing (Zeroing)
To protect against memory-dump attacks, the system utilizes a custom implementation of the `Drop` trait. When any security buffer goes out of scope:
- The raw data is overwritten with zeros (0x00).
- The memory is deallocated immediately.

## 3. Offline Integrity
The applications are designed to work entirely **Offline**. There are no network calls, telemetry, or hidden trackers. Your data never leaves your machine.

## 4. Open Source Transparency
The source code is open for peer review. We believe in "Security through Transparency," allowing anyone to audit the logic in `engine.rs`, `sender.rs`, and `receiver.rs`.
