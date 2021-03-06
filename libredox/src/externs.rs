use core::fmt;
use core::fmt::Write;
use core::ptr;
use core::result;

use syscall::{sys_debug, sys_exit};

pub struct DebugStream;

impl Write for DebugStream {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            unsafe { sys_debug(b) };
        }

        result::Result::Ok(())
    }
}

#[lang="panic_fmt"]
#[allow(unused_must_use)]
pub extern fn panic_impl(args: fmt::Arguments, file: &'static str, line: u32) -> ! {
    let mut stream = DebugStream;
    fmt::write(&mut stream, args);
    fmt::write(&mut stream, format_args!(" in {}:{}\n", file, line));

    unsafe {
        sys_exit(-1);
        loop {
            asm!("sti");
            asm!("hlt");
        }
    }
}

#[lang="stack_exhausted"]
extern "C" fn stack_exhausted() {

}

#[lang="eh_personality"]
extern "C" fn eh_personality() {

}

#[no_mangle]
pub unsafe extern "C" fn memcmp(a: *mut u8, b: *const u8, len: usize) -> isize {
    for i in 0..len {
        let c_a = ptr::read(a.offset(i as isize));
        let c_b = ptr::read(b.offset(i as isize));
        if c_a != c_b {
            return c_a as isize - c_b as isize;
        }
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dst: *mut u8, src: *const u8, len: usize) {
    if src < dst {
        asm!("std
            rep movsb"
            :
            : "{edi}"(dst.offset(len as isize - 1)), "{esi}"(src.offset(len as isize - 1)), "{ecx}"(len)
            : "cc", "memory"
            : "intel", "volatile");
    } else {
        asm!("cld
            rep movsb"
            :
            : "{edi}"(dst), "{esi}"(src), "{ecx}"(len)
            : "cc", "memory"
            : "intel", "volatile");
    }
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, len: usize) {
    asm!("cld
        rep movsb"
        :
        : "{edi}"(dst), "{esi}"(src), "{ecx}"(len)
        : "cc", "memory"
        : "intel", "volatile");
}

#[no_mangle]
pub unsafe extern "C" fn memset(dst: *mut u8, c: i32, len: usize) {
    asm!("cld
        rep stosb"
        :
        : "{eax}"(c), "{edi}"(dst), "{ecx}"(len)
        : "cc", "memory"
        : "intel", "volatile");
}

#[no_mangle]
//TODO Make this better
/// 64 bit remainder on 32 bit arch
pub extern "C" fn __umoddi3(a: u64, b: u64) -> u64 {
    if b == 0 {
        return 0;
    }

    let mut rem = a;
    while rem >= b {
        rem -= b;
    }
    rem
}

#[no_mangle]
//TODO Make this better
/// 64 bit division on 32 bit arch
pub extern "C" fn __udivdi3(a: u64, b: u64) -> u64 {
    if b == 0 {
        return 0;
    }

    let mut quot = 0;
    let mut rem = a;
    while rem >= b {
        rem -= b;
        quot += 1;
    }
    quot
}

/*
pub fn unsupported() {
    unsafe { asm!("int 3" : : : : "intel", "volatile") }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern fn fmod(x: f64, y: f64) -> f64 {
    unsupported();
    return 0.0;
}

#[allow(unused_variables)]
#[no_mangle]
pub extern fn fmodf(x: f32, y: f32) -> f32 {
    unsupported();
    return 0.0;
}

#[allow(unused_variables)]
#[no_mangle]
pub extern fn __powisf2(a: f32, x: i32) -> f32 {
    unsupported();
    return 0.0;
}

#[allow(unused_variables)]
#[no_mangle]
pub extern fn __powidf2(a: f64, x: i32) -> f64 {
    unsupported();
    return 0.0;
}

#[no_mangle]
pub extern fn __mulodi4(a: i32, b: i32, overflow: *mut i32) -> i32 {
    let result = (a as i64) * (b as i64);
    if result > 2 << 32 {
        unsafe {
            ptr::write(overflow, 1);
        }
    }
    return result as i32;
}

#[no_mangle]
pub extern fn __moddi3(a: i32, b: i32) -> i32 {
    return a%b;
}

#[no_mangle]
pub extern fn __divdi3(a: i32, b: i32) -> i32 {
    return a/b;
}
*/
