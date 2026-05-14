// SPDX-License-Identifier: MIT
// LunarOS - 内核库入口
// 版权所有 (c) 2024 LunarOS 贡献者

#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub mod arch;
pub mod serial;

/// 内核初始化入口
///
/// 在 `_start` 中首先调用，完成各子系统的早期初始化。
pub fn init() {
    arch::x86_64::init();
}

// ──────────────────────────────────────────
// 测试框架
// ──────────────────────────────────────────

/// 可测试特征：每个测试用例实现此特征
pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

/// 测试运行器：遍历所有测试并执行
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("[TEST] 运行 {} 个测试", tests.len());
    for test in tests {
        test.run();
    }
    // 通知 QEMU 测试通过并退出（exit code 33 = (0x10 << 1) | 1）
    exit_qemu(QemuExitCode::Success);
}

/// 测试模式下的 panic 处理
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[FAILED]");
    serial_println!("[TEST] panic: {}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {
        x86_64::instructions::hlt();
    }
}

/// QEMU 退出状态码
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// 通过 I/O 端口通知 QEMU 退出
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// ──────────────────────────────────────────
// 测试模式入口（cargo test）
// ──────────────────────────────────────────

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

// ──────────────────────────────────────────
// 基础测试用例
// ──────────────────────────────────────────

#[test_case]
fn trivial_assertion() {
    assert_eq!(1 + 1, 2);
}
