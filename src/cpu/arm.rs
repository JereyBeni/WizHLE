//! ARM (AArch32 / AArch64) CPU backend placeholder.
//!
//! In a real HLE emulator this module would implement instruction decoding,
//! register state, and high-level system call / API interception relevant
//! to Android and TouchWiz layers.

#![allow(dead_code)]

pub struct ArmCore {
    // Placeholder for general-purpose registers, CPSR/SPSR, etc.
}

impl ArmCore {
    pub fn new() -> Self {
        Self {}
    }
}
