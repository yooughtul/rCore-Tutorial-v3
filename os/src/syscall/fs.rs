use crate::{batch::{get_current_app_range, get_user_stack_range}, console::print};

//use super::process::sys_exit;

const FD_STDOUT: usize = 1;
/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let app_range = get_current_app_range();
    let stack_range = get_user_stack_range();
    let buf_begin = buf as usize;
    let buf_end = unsafe { buf.offset(len as isize) } as usize;

   // 检查是否完全在应用空间内
    let in_app_space = (buf_begin >= app_range.0 && buf_begin < app_range.1) 
    && (buf_end >= app_range.0 && buf_end <= app_range.1);

    // 检查是否完全在用户栈空间内
    let in_user_stack = (buf_begin >= stack_range.0 && buf_begin < stack_range.1) 
    && (buf_end >= stack_range.0 && buf_end <= stack_range.1);

// 任何跨边界访问都应该返回错误
if !in_app_space && !in_user_stack {
   println!("[kernel] Invalid buffer range!");
   println!("[kernel] buf: [{:#x}, {:#x})", buf_begin, buf_end);
   println!("[kernel] app: [{:#x}, {:#x})", app_range.0, app_range.1);
   println!("[kernel] stack: [{:#x}, {:#x})", stack_range.0, stack_range.1);
   return -1;
}

    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            -1 as isize
        }
    }
}