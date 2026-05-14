# 🌙 LunarOS

[![CI](https://github.com/your-org/lunaros/actions/workflows/ci.yml/badge.svg)](https://github.com/your-org/lunaros/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust: nightly](https://img.shields.io/badge/rust-nightly-orange.svg)](https://www.rust-lang.org/)
[![Platform: x86_64](https://img.shields.io/badge/platform-x86__64-blue.svg)]()

---

**中文** | [English](#english)

---

## 简介

LunarOS 是一个用 **Rust** 从零编写的开源操作系统内核，由中国开源社区主导开发。

我们相信：内存安全、高性能、开源透明，是未来操作系统的基础。LunarOS 以月相命名各阶段里程碑——从新月到满月，象征从无到有的成长历程。长期目标是打造一个面向通用计算的、可对标 Linux 的操作系统内核。

## 当前状态

**v0.1 Crescent（新月）** — 正在开发中

- [x] 项目骨架与构建系统
- [x] 串口输出（早期调试）
- [x] x86_64 启动流程
- [x] 基础中断处理（IDT）
- [ ] 内存管理（分页）
- [ ] 堆内存分配器

## 快速开始

### 环境依赖

- Rust nightly（含 `rust-src`、`llvm-tools-preview` 组件）
- QEMU（`qemu-system-x86_64`）
- `bootimage` 工具

### 安装依赖

```bash
# 安装 Rust nightly
rustup install nightly
rustup default nightly
rustup component add rust-src llvm-tools-preview

# 安装 bootimage
cargo install bootimage

# macOS
brew install qemu

# Ubuntu/Debian
sudo apt install qemu-system-x86
```

### 编译与运行

```bash
git clone https://github.com/your-org/lunaros.git
cd lunaros

make run
```

预期输出：

```
  ╔══════════════════════════════════════════╗
  ║         LunarOS v0.1 Crescent            ║
  ║      新月 - 万里之行，始于足下           ║
  ╚══════════════════════════════════════════╝

[BOOT] LunarOS v0.1 Crescent - Booting...
[BOOT] 物理内存映射已加载
[BOOT] 内核初始化完成，系统就绪
```

### 运行测试

```bash
make test
```

## 路线图

| 版本 | 代号 | 目标 | 状态 |
|------|------|------|------|
| v0.1 | Crescent 新月 | 启动 + 串口输出 | 🔧 开发中 |
| v0.2 | Waxing 眉月 | 内存管理（分页 + 堆分配器）| 📋 规划中 |
| v0.3 | Quarter 上弦月 | 进程与线程模型 | 📋 规划中 |
| v0.4 | Gibbous 凸月 | 虚拟文件系统（VFS）| 📋 规划中 |
| v0.5 | Full 满月 | 基础 Shell + 系统调用 | 📋 规划中 |
| v0.6 | Waning 下弦月 | 网络栈（TCP/IP）| 📋 规划中 |
| v1.0 | Eclipse 日蚀 | 可自举的完整内核 | 🌠 愿景 |

## 如何贡献

我们欢迎所有形式的贡献！

1. Fork 本仓库
2. 创建特性分支：`git checkout -b feat/my-feature`
3. 提交变更：`git commit -m 'feat: add some feature'`
4. 推送分支：`git push origin feat/my-feature`
5. 发起 Pull Request

请阅读 [贡献指南](CONTRIBUTING.md) 和 [行为准则](CODE_OF_CONDUCT.md)。

## 许可证

本项目采用 [MIT 许可证](LICENSE)。

---

<a name="english"></a>

## English

### Introduction

LunarOS is an open-source operating system kernel written from scratch in **Rust**, led by the Chinese open-source community.

We believe that memory safety, high performance, and open transparency are the foundation of future operating systems. LunarOS names its milestones after lunar phases — from crescent to full moon — symbolizing growth from nothing. The long-term goal is to build a general-purpose OS kernel comparable to Linux.

### Current Status

**v0.1 Crescent** — In Development

- [x] Project skeleton and build system
- [x] Serial output (early debug)
- [x] x86_64 boot flow
- [x] Basic interrupt handling (IDT)
- [ ] Memory management (paging)
- [ ] Heap allocator

### Quick Start

```bash
# Install Rust nightly
rustup install nightly && rustup default nightly
rustup component add rust-src llvm-tools-preview
cargo install bootimage

# macOS
brew install qemu
# Ubuntu
sudo apt install qemu-system-x86

# Build and run
git clone https://github.com/your-org/lunaros.git
cd lunaros && make run
```

### Roadmap

| Version | Codename | Goal | Status |
|---------|----------|------|--------|
| v0.1 | Crescent | Boot + serial output | 🔧 In Progress |
| v0.2 | Waxing | Memory management | 📋 Planned |
| v0.3 | Quarter | Process & thread model | 📋 Planned |
| v0.4 | Gibbous | Virtual filesystem (VFS) | 📋 Planned |
| v0.5 | Full | Basic shell + syscalls | 📋 Planned |
| v0.6 | Waning | Network stack (TCP/IP) | 📋 Planned |
| v1.0 | Eclipse | Self-hosting kernel | 🌠 Vision |

### License

This project is licensed under the [MIT License](LICENSE).
