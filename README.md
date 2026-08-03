# WizHLE

**WizHLE** is an experimental High-Level Emulator (HLE) project focused on the Samsung TouchWiz Android user interface, written in Rust.

> **Important notice**  
> This is currently a **skeleton / educational project**. It does **not** contain a functional Android or TouchWiz emulator. Building a real HLE Android emulator is an extremely complex undertaking that requires significant time, reverse-engineering effort, and deep knowledge of Android internals, Samsung proprietary layers, and hardware emulation.

## Goals (long-term)

- High-Level Emulation (HLE) approach focused on TouchWiz / Nature UX interfaces
- Support for ARM and x86 guest architectures
- Written primarily in Rust for memory safety and performance
- Modular design: CPU core, memory management, graphics abstraction, input, and TouchWiz-specific UI layer
- **64HLE**: dedicated support for 64-bit applications (AArch64 / x86_64)
- Integrated **APK installer** for sideloading Android applications

## Supported architectures (planned)

| Architecture              | Status  | Notes                                          |
|---------------------------|---------|------------------------------------------------|
| ARM (AArch32)             | Planned | Classic 32-bit Samsung devices                 |
| ARM (AArch64)             | Planned | Primary 64-bit target (via 64HLE)              |
| x86                       | Planned | 32-bit compatibility                           |
| x86_64                    | Planned | 64-bit target (via 64HLE)                      |

## 64HLE (planned)

**64HLE** is the planned subsystem dedicated to the correct execution and high-level emulation of **64-bit applications**.

Its main responsibilities will include:

- Proper handling of 64-bit address spaces and register sets (AArch64 / x86_64)
- Support for 64-bit Android ABIs and system libraries
- Compatibility layer for 64-bit native code used by modern TouchWiz / One UI era applications
- Integration with the main WizHLE core while keeping 32-bit and 64-bit paths clearly separated where necessary

64HLE is currently in the planning stage and has no implementation yet.

## APK Installer (planned)

WizHLE is planned to include a built-in APK installer that will allow users to sideload Android application packages into the emulated environment.

## Related projects (HLE family)

| Project       | Description                                      | Repository                              |
|---------------|--------------------------------------------------|-----------------------------------------|
| **WizHLE**    | Samsung TouchWiz + APK installer                 | [WizHLE](https://github.com/JereyBeni/WizHLE) |
| **DesktopHLE**| macOS applications on Windows                    | [DesktopHLE](https://github.com/JereyBeni/DesktopHLE) |
| **WinHLE**    | Windows XP / 7 applications on Windows 10/11     | [WinHLE](https://github.com/JereyBeni/WinHLE) |
| **TVHLE**     | Android TV on Windows 10/11                      | [TVHLE](https://github.com/JereyBeni/TVHLE) |

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
- 64HLE and APK installer documented as planned components (no code yet)
- No functional emulation yet

## Disclaimer

This project is for educational and research purposes only. Samsung TouchWiz is proprietary software. Any reverse-engineering activities must comply with applicable laws and licenses.

## License

MIT License.
