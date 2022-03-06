#![no_std]
#![no_main]

mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello, world! {}", ":D");

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
