//! Trace syscall implementation

use alloc::collections::BTreeMap;
use lazy_static::*;
use crate::sync::UPSafeCell;

lazy_static! {
    /// 系统调用计数器
    static ref SYSCALL_COUNTS: UPSafeCell<BTreeMap<usize, usize>> = unsafe {
        UPSafeCell::new(BTreeMap::new())
    };
}

/// 增加特定系统调用的计数
fn increment_syscall_count(syscall_id: usize) -> usize {
    let mut counts = SYSCALL_COUNTS.exclusive_access();
    let count = counts.entry(syscall_id).or_insert(0);
    *count += 1;
    *count
}

/// 尝试增加特定系统调用的计数（供其他模块使用）
pub fn try_increment_syscall_count(syscall_id: usize) -> Option<usize> {
    let mut counts = SYSCALL_COUNTS.exclusive_access();
    let count = counts.entry(syscall_id).or_insert(0);
    *count += 1;
    Some(*count)
}

/// Trace system call implementation
/// 
/// 如果 trace_request 为 0，则 id 应被视作 *const u8 ，表示读取当前任务 id 地址处一个字节的无符号整数值。
/// 如果 trace_request 为 1，则 id 应被视作 *const u8 ，表示写入 data（作为 u8）到 id 地址处。
/// 如果 trace_request 为 2，表示查询当前任务调用编号为 id 的系统调用的次数。
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace - request: {}, id: {}, data: {}", trace_request, id, data);
    
    match trace_request {
        // 读取id地址处的u8值
        0 => {
            let ptr = id as *const u8;
            unsafe {
                if let Some(value) = ptr.as_ref() {
                    trace!("Trace: 读取地址 {:p} 处的值: {}", ptr, *value);
                    *value as isize
                } else {
                    trace!("Trace: 无法读取地址 {:p}", ptr);
                    -1
                }
            }
        },
        // 写入data的最低字节到id地址处
        1 => {
            let ptr = id as *mut u8;
            let value = (data & 0xff) as u8; // 只取最低字节
            unsafe {
                if let Some(target) = ptr.as_mut() {
                    trace!("Trace: 写入值 {} 到地址 {:p}", value, ptr);
                    *target = value;
                    0
                } else {
                    trace!("Trace: 无法写入地址 {:p}", ptr);
                    -1
                }
            }
        },
        // 查询系统调用次数
        2 => {
            // 先增加当前系统调用的计数
            let count = increment_syscall_count(id);
            trace!("Trace: 系统调用 {} 的计数为 {}", id, count);
            count as isize
        },
        _ => {
            trace!("Trace: 未知请求类型: {}", trace_request);
            -1
        }
    }
}
