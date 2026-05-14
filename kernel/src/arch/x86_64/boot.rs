// SPDX-License-Identifier: MIT
// LunarOS - x86_64 启动初始化

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;

lazy_static! {
    /// 中断描述符表（IDT）
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        // 注册断点异常处理器（用于调试）
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        // 注册双重故障处理器
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    };
}

/// x86_64 启动序列初始化
pub fn init() {
    // 加载中断描述符表
    IDT.load();
}

/// 断点异常处理器
///
/// 收到 INT3 断点异常时打印寄存器信息，然后继续执行。
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    crate::serial_println!("[EXCEPTION] 断点异常\n{:#?}", stack_frame);
}

/// 双重故障处理器
///
/// 双重故障是不可恢复的，打印信息后停机。
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("[EXCEPTION] 双重故障\n{:#?}", stack_frame);
}

// ──────────────────────────────────────────
// 测试
// ──────────────────────────────────────────

#[test_case]
fn test_breakpoint_exception() {
    // 触发断点异常，验证 IDT 正常工作
    x86_64::instructions::interrupts::int3();
}
