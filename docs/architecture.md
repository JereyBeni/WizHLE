# WizHLE Architecture Overview

## Design philosophy

WizHLE aims to use a **High-Level Emulation (HLE)** approach rather than full Low-Level Emulation (LLE) of every hardware component. The focus is on correctly implementing the observable behavior of the TouchWiz / Nature UX software stack and the Android services it depends on.

## Planned major components

1. **CPU backends** (`src/cpu/`)
   - ARM (AArch32 and AArch64) as primary target
   - x86 / x86_64 as secondary target
   - Instruction decoding and execution will be added gradually
   - System call and binder-level interception for HLE

2. **Memory manager** (`src/memory/`)
   - Guest physical memory allocation
   - Simple virtual memory mapping (later page tables / MMU)
   - Shared memory regions for graphics and IPC
   - Explicit support for both 32-bit and 64-bit address spaces

3. **Graphics** (`src/graphics/`)
   - High-level rendering of TouchWiz UI elements
   - Avoid full OpenGL ES / Vulkan emulation initially
   - SurfaceFlinger / Hardware Composer abstraction where useful

4. **Input** (`src/input/`)
   - Translation of host events into Android InputEvent stream
   - Support for multi-touch gestures typical of TouchWiz

5. **TouchWiz layer** (`src/touchwiz/`)
   - Launcher (TouchWiz Home)
   - Widgets and live wallpapers
   - Notification panel and quick settings behavior
   - Samsung-specific services (when reverse-engineered and legally permissible)

6. **64HLE** (planned subsystem)
   - Dedicated high-level support for **64-bit applications**
   - Targets AArch64 and x86_64 ABIs
   - Handles 64-bit register sets, larger address spaces and 64-bit native libraries
   - Keeps a clear separation from 32-bit execution paths where required
   - Intended to improve compatibility with applications from the later TouchWiz and early One UI eras that rely on 64-bit code

## Current limitations

- No instruction execution
- No Android system image loading
- No graphics output
- No input handling beyond stubs
- No TouchWiz functionality
- 64HLE is documented but not yet implemented

This document will be expanded as the project progresses.
