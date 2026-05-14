// SPDX-License-Identifier: MIT
// LunarOS - x86_64 架构模块

pub mod boot;

/// x86_64 架构初始化
///
/// 依次完成：GDT、IDT 等基础硬件初始化。
pub fn init() {
    boot::init();
}
