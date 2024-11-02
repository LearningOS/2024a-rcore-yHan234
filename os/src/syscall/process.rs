//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    mm::translated_byte_buffer,
    task::{
        change_program_brk, current_mmap, current_munmap, current_user_token,
        exit_current_and_run_next, get_current_task_control_block, suspend_current_and_run_next,
        TaskStatus,
    },
    timer::{get_time_ms, get_time_us},
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");

    let buffers = translated_byte_buffer(
        current_user_token(),
        _ts as *const u8,
        core::mem::size_of::<TimeVal>(),
    );
    assert!(buffers.len() == 1); // 未考虑跨页

    let us = get_time_us();

    unsafe {
        *(buffers[0].as_ptr() as *mut TimeVal) = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }

    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");

    let buffers = translated_byte_buffer(
        current_user_token(),
        _ti as *const u8,
        core::mem::size_of::<TaskInfo>(),
    );
    assert!(buffers.len() == 1); // 未考虑跨页

    let tcb = get_current_task_control_block();

    unsafe {
        *(buffers[0].as_ptr() as *mut TaskInfo) = TaskInfo {
            status: (*tcb).task_status,
            syscall_times: (*tcb).syscall_times,
            time: get_time_ms() - (*tcb).task_start_time,
        };
    }

    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap");
    current_mmap(_start, _len, _port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap");
    current_munmap(_start, _len)
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
