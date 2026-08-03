# WizHLE

**WizHLE** is an experimental High-Level Emulator (HLE) project focused on the Samsung TouchWiz Android user interface, written in Rust.

> **Important notice**  
> This is currently a **skeleton / educational project**. It does **not** contain a functional Android or TouchWiz emulator. Building a real HLE Android emulator is an extremely complex undertaking that requires significant time, reverse-engineering effort, and deep knowledge of Android internals, Samsung proprietary layers, and hardware emulation.

## Goals (long-term)

- High-Level Emulation (HLE) approach focused on TouchWiz / Nature UX interfaces
- Support for ARM and x86 guest architectures
- Written primarily in Rust for memory safety and performance
- Modular design: CPU core, memory management, graphics abstraction, input, and TouchWiz-specific UI layer

## Supported architectures (planned)

| Architecture              | Status  | Notes                                          |
|---------------------------|---------|------------------------------------------------|
| ARM (AArch32 / AArch64)   | Planned | Primary target for classic Samsung devices     |
| x86 / x86_64              | Planned | Secondary target for broader compatibility     |

## Project structure

```
WizHLE/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   ├── cpu/
│   │   ├── mod.rs
│   │   ├── arm.rs
│   │   └── x86.rs
│   ├── memory/
│   │   └── mod.rs
│   ├── graphics/
│   │   └── mod.rs
│   ├── input/
│   │   └── mod.rs
│   └── touchwiz/
│       └── mod.rs
└── docs/
    └── architecture.md
```

## Building

```bash
cargo build
cargo run
```

## Current status

- Basic project skeleton
- Placeholder modules for CPU (ARM / x86), memory, graphics, input and TouchWiz layer
- No functional emulation yet

## Disclaimer

This project is for educational and research purposes only. Samsung TouchWiz is proprietary software. Any reverse-engineering activities must comply with applicable laws and licenses.

## License

MIT License (planned).
