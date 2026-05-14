// SPDX-License-Identifier: MIT
// LunarOS - 串口驱动（用于早期调试输出）

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    /// 全局串口实例，COM1（0x3F8）
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

/// 向串口写入格式化字符串（不换行）
#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    // 在关中断状态下操作，避免死锁
    interrupts::without_interrupts(|| {
        SERIAL1.lock().write_fmt(args).expect("串口写入失败");
    });
}

/// 串口打印宏（不换行）
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// 串口打印宏（换行）
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*
    ));
}
