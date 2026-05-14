// SPDX-License-Identifier: MIT
// LunarOS - 用 Rust 编写的开源操作系统内核
// 版权所有 (c) 2024 LunarOS 贡献者

#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 级别的入口点

use core::panic::PanicInfo;
use lunar_kernel::serial_println;

/// 内核入口点
///
/// 由 bootloader 调用，传入引导信息。
/// 函数名 `_start` 是链接器约定的入口符号。
#[no_mangle]
pub extern "C" fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
    // 初始化串口，用于早期调试输出
    lunar_kernel::init();

    // 打印启动横幅
    serial_println!("  ╔══════════════════════════════════════════╗");
    serial_println!("  ║         LunarOS v0.1 Crescent            ║");
    serial_println!("  ║      新月 - 万里之行，始于足下           ║");
    serial_println!("  ╚══════════════════════════════════════════╝");
    serial_println!("");
    serial_println!("[BOOT] LunarOS v0.1 Crescent - Booting...");
    serial_println!("[BOOT] 物理内存映射已加载");
    serial_println!("[BOOT] 内核初始化完成，系统就绪");

    // 内核主循环：暂停 CPU 等待中断（当前阶段无其他任务）
    loop {
        x86_64::instructions::hlt();
    }
}

/// panic 处理函数
///
/// 当内核发生不可恢复错误时调用。
/// 打印错误信息后停机。
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[PANIC] 内核 panic: {}", info);
    // 停机
    loop {
        x86_64::instructions::hlt();
    }
}
