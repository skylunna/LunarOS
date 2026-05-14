# LunarOS Getting Started Guide

This guide walks you through setting up the LunarOS development environment on Ubuntu or macOS from scratch.

---

## 1. Requirements

| Component | Minimum Version | Notes |
|-----------|----------------|-------|
| Rust | nightly-2024-01-01+ | Requires `rust-src` and `llvm-tools-preview` |
| QEMU | 7.0+ | `qemu-system-x86_64` |
| bootimage | 0.10+ | Cargo tool for creating bootable images |
| Git | 2.x | Version control |

---

## 2. Install Rust

### 2.1 Install rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### 2.2 Install and set nightly toolchain

LunarOS uses nightly features (`abi_x86_interrupt`, `custom_test_frameworks`, etc.):

```bash
rustup install nightly
rustup default nightly
```

### 2.3 Install required components

```bash
# rust-src: needed to compile core library for custom targets
# llvm-tools-preview: needed for bootimage
rustup component add rust-src llvm-tools-preview
```

### 2.4 Verify installation

```bash
rustc --version   # should show nightly
cargo --version
```

---

## 3. Install bootimage

`bootimage` links the compiled kernel with a bootloader to produce a bootable disk image:

```bash
cargo install bootimage
```

---

## 4. Install QEMU

### macOS

```bash
brew install qemu
```

### Ubuntu / Debian

```bash
sudo apt-get update
sudo apt-get install -y qemu-system-x86
```

### Verify

```bash
qemu-system-x86_64 --version
```

---

## 5. Clone the Repository

```bash
git clone https://github.com/your-org/lunaros.git
cd lunaros
```

---

## 6. Build the Kernel

```bash
make build
```

Or directly with Cargo:

```bash
cd kernel
cargo build
```

The first build downloads dependencies and takes 1–3 minutes.

---

## 7. Run in QEMU

```bash
make run
```

You should see the following serial output:

```
  ╔══════════════════════════════════════════╗
  ║         LunarOS v0.1 Crescent            ║
  ║      新月 - 万里之行，始于足下           ║
  ╚══════════════════════════════════════════╝

[BOOT] LunarOS v0.1 Crescent - Booting...
[BOOT] Physical memory map loaded
[BOOT] Kernel initialization complete, system ready
```

Press `Ctrl+A` then `X` to quit QEMU.

---

## 8. Run Tests

```bash
make test
```

Tests run inside a headless QEMU instance and report results via a special I/O port exit mechanism.

---

## 9. Troubleshooting

### `error: no override and no default toolchain set`

```bash
rustup default nightly
```

### `bootimage: command not found`

```bash
cargo install bootimage
export PATH="$HOME/.cargo/bin:$PATH"
```

### No output from QEMU

Verify QEMU is installed:

```bash
which qemu-system-x86_64
```

### `can't find crate for 'core'`

```bash
rustup component add rust-src
```

---

## 10. Development Workflow

```bash
make build    # Recompile after changes
make run      # Test in QEMU
make test     # Run unit tests
make fmt      # Format code
make clippy   # Static analysis
```

---

Next: Read the [Architecture docs](../architecture.md) to understand the kernel internals.
