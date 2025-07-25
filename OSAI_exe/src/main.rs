use std::fs::File;
use std::io::Read;
use std::mem;
use libc::{mmap, PROT_EXEC, PROT_READ, PROT_WRITE, MAP_ANON, MAP_PRIVATE};

fn main() {
    let mut f = File::open("hello.bin").unwrap();
    let mut code = vec![];
    f.read_to_end(&mut code).unwrap();

    let exec_mem = unsafe {
        mmap(
            std::ptr::null_mut(),
            code.len(),
            PROT_READ | PROT_WRITE | PROT_EXEC,
            MAP_ANON | MAP_PRIVATE,
            -1,
            0,
        )
    };

    unsafe {
        std::ptr::copy_nonoverlapping(code.as_ptr(), exec_mem as *mut u8, code.len());

        let func: extern "C" fn() = mem::transmute(exec_mem);
        func();
    }
}

