//! Trace syscall implementation

use crate::task::current_task;

/// 尝试增加特定系统调用的计数（供其他模块使用）
pub fn try_increment_syscall_count(syscall_id: usize) -> Option<usize> {
    let mut task = current_task();
    task.task_syscall_counts[syscall_id] += 1;
    Some(task.task_syscall_counts[syscall_id])
}

/// Trace system call implementation
///
/// 如果 trace_request 为 0，则 id 应被视作 *const u8 ，表示读取当前任务 id 地址处一个字节的无符号整数值。
/// 如果 trace_request 为 1，则 id 应被视作 *const u8 ，表示写入 data（作为 u8）到 id 地址处。
/// 如果 trace_request 为 2，表示查询当前任务调用编号为 id 的系统调用的次数。
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    match trace_request {
        // 读取id地址处的u8值
        0 => {
            let ptr = id as *const u8;
            unsafe {
                if let Some(value) = ptr.as_ref() {
                    *value as isize
                } else {
                    -1
                }
            }
        }
        // 写入data的最低字节到id地址处
        1 => {
            let ptr = id as *mut u8;
            let value = (data & 0xff) as u8; // 只取最低字节
            unsafe {
                if let Some(target) = ptr.as_mut() {
                    *target = value;
                    0
                } else {
                    -1
                }
            }
        }
        // 查询系统调用次数
        2 => {
            let task = current_task();
            task.task_syscall_counts[id] as isize
        }
        _ => -1,
    }
}
