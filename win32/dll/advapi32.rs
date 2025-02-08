#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &::core::panic::PanicInfo) -> ! {
    loop {}
}

#[link(
    name = "retrowin32",
    kind = "raw-dylib",
    import_name_type = "undecorated"
)]
extern "stdcall" {
    fn retrowin32_syscall();
}

#[no_mangle]
pub unsafe extern "stdcall" fn RegCloseKey(_: u32) {
    retrowin32_syscall();
}

#[no_mangle]
pub unsafe extern "stdcall" fn RegOpenKey(_: u32) {
    retrowin32_syscall();
}

// Have to wrap the pointers in a struct to impl Sync.
// Have to impl Sync to use in a static.
// The other option is a "static mut" but that creates a .data section we don't otherwise need.
#[repr(transparent)]
pub struct VTableEntry(*const fn());
unsafe impl Sync for VTableEntry {}

#[no_mangle]
pub static vtab: [VTableEntry; 4] = [
    VTableEntry(core::ptr::null()),
    VTableEntry(RegCloseKey as _),
    VTableEntry(core::ptr::null()),
    VTableEntry(RegOpenKey as _),
];

// core::arch::global_asm!(
//     ".globl _IDirectSound",
//     "_IDirectSound:",
//     ".long _RegCloseKey",
//     ".long 0",
// );
