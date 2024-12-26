#![no_std]

use core::panic::PanicInfo;
use core::ptr;

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    // NEW!
    // Initialize RAM
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }

    let count = &raw const _ebss as usize - &raw const _sbss as usize;
    ptr::write_bytes(&raw mut _sbss, 0, count);

    let count = &raw const _edata as usize - &raw const _sdata as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &raw mut _sdata, count);

    // Call user entry point
    extern "Rust" {
        fn main() -> !;
    }

    main()
}

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            // type check the given path
            let f: fn() -> ! = $path;

            f()
        }
    }
}
