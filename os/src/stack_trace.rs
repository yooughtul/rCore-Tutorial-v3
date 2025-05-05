use core::{arch::asm,ptr};

pub unsafe fn stack_trace()->(){
    let mut fp:*const usize;//指向一个不可变的数据，但是允许指针修改别处
    unsafe{asm!("mv {},fp",out(reg) fp)};

    println!("Stack trace:");
    while fp!=ptr::null(){
        let ra=unsafe { *fp.sub(1) };
        let prev_fp=unsafe { *fp.sub(2) };
        println!("Return Address:{:#x},Prev Fp:{:#x}",ra,prev_fp);
        fp=prev_fp as *const usize;
    }
    println!("End stack trace");

}