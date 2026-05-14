# LunarOS 环境搭建指南（中文）

本文档指导你在 Ubuntu 或 macOS 上从零搭建 LunarOS 开发环境。

---

## 一、系统要求

| 组件 | 最低版本 | 说明 |
|------|----------|------|
| Rust | nightly-2024-01-01+ | 需要 `rust-src` 和 `llvm-tools-preview` |
| QEMU | 7.0+ | `qemu-system-x86_64` |
| bootimage | 0.10+ | Cargo 工具，用于创建可引导镜像 |
| Git | 2.x | 版本控制 |

---

## 二、安装 Rust

### 2.1 安装 rustup（Rust 工具链管理器）

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### 2.2 安装并设置 nightly 工具链

LunarOS 使用 nightly 特性（如 `abi_x86_interrupt`、`custom_test_frameworks`）：

```bash
rustup install nightly
rustup default nightly
```

### 2.3 安装必要组件

```bash
# rust-src：编译自定义目标时需要标准库源码
# llvm-tools-preview：生成内核镜像所需
rustup component add rust-src llvm-tools-preview
```

### 2.4 验证安装

```bash
rustc --version   # 应显示 nightly 版本
cargo --version
```

---

## 三、安装 bootimage

`bootimage` 将编译好的内核链接到 bootloader，生成可引导的磁盘镜像：

```bash
cargo install bootimage
```

---

## 四、安装 QEMU

### macOS

```bash
# 使用 Homebrew
brew install qemu
```

### Ubuntu / Debian

```bash
sudo apt-get update
sudo apt-get install -y qemu-system-x86
```

### 验证 QEMU 安装

```bash
qemu-system-x86_64 --version
```

---

## 五、克隆项目

```bash
git clone https://github.com/your-org/lunaros.git
cd lunaros
```

---

## 六、编译内核

```bash
make build
```

或者直接使用 Cargo：

```bash
cd kernel
cargo build
```

首次编译会下载依赖，耗时约 1-3 分钟。

---

## 七、在 QEMU 中运行

```bash
make run
```

预期看到以下串口输出：

```
  ╔══════════════════════════════════════════╗
  ║         LunarOS v0.1 Crescent            ║
  ║      新月 - 万里之行，始于足下           ║
  ╚══════════════════════════════════════════╝

[BOOT] LunarOS v0.1 Crescent - Booting...
[BOOT] 物理内存映射已加载
[BOOT] 内核初始化完成，系统就绪
```

按 `Ctrl+A` 然后 `X` 退出 QEMU。

---

## 八、运行测试

```bash
make test
```

测试会在无头 QEMU 中运行，通过特殊 I/O 端口通知退出状态。

---

## 九、常见问题

### Q: `error: no override and no default toolchain set`

```bash
rustup default nightly
```

### Q: `bootimage: command not found`

```bash
cargo install bootimage
# 确保 ~/.cargo/bin 在 PATH 中
export PATH="$HOME/.cargo/bin:$PATH"
```

### Q: QEMU 无输出

确认 `qemu-system-x86_64` 已安装：

```bash
which qemu-system-x86_64
```

### Q: 编译报错 `can't find crate for 'core'`

```bash
rustup component add rust-src
```

---

## 十、开发工作流

```bash
# 修改代码后
make build          # 重新编译
make run            # 在 QEMU 中测试
make test           # 运行单元测试
make fmt            # 格式化代码
make clippy         # 静态分析
```

---

下一步：阅读[架构文档](../architecture.md)了解内核内部结构。
