# SPDX-License-Identifier: MIT
# LunarOS 构建系统

KERNEL_DIR := kernel
TARGET     := x86_64-unknown-none
KERNEL_BIN := $(KERNEL_DIR)/target/$(TARGET)/debug/lunar-kernel

# QEMU 参数：串口输出到终端，无图形界面
QEMU_ARGS := \
	-machine q35 \
	-cpu qemu64 \
	-m 128M \
	-serial stdio \
	-display none \
	-no-reboot \
	-no-shutdown

.PHONY: all build run clean test fmt clippy

all: build

## 编译内核
build:
	@echo ">>> 编译 LunarOS 内核..."
	cd $(KERNEL_DIR) && cargo build
	@echo ">>> 构建完成: $(KERNEL_BIN)"

## 编译并在 QEMU 中运行
run: build
	@echo ">>> 启动 QEMU..."
	cd $(KERNEL_DIR) && cargo run

## 运行测试
test:
	@echo ">>> 运行内核测试..."
	cd $(KERNEL_DIR) && cargo test

## 格式化代码
fmt:
	cd $(KERNEL_DIR) && cargo fmt

## 静态分析
clippy:
	cd $(KERNEL_DIR) && cargo clippy

## 清理构建产物
clean:
	@echo ">>> 清理构建产物..."
	cd $(KERNEL_DIR) && cargo clean
	@echo ">>> 清理完成"

## 显示帮助
help:
	@echo ""
	@echo "LunarOS 构建命令:"
	@echo "  make build   - 编译内核"
	@echo "  make run     - 在 QEMU 中运行"
	@echo "  make test    - 运行测试"
	@echo "  make fmt     - 格式化代码"
	@echo "  make clippy  - 静态分析"
	@echo "  make clean   - 清理构建产物"
	@echo ""
