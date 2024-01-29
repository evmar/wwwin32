#![no_std]
#![no_main]
#![windows_subsystem = "console"]

#[allow(unused_imports)]
use core::arch::asm;

use windows_sys::Win32::{
    Storage::FileSystem::WriteFile,
    System::Console::{GetStdHandle, STD_OUTPUT_HANDLE},
};

fn print(bufs: &[&[u8]]) {
    unsafe {
        let stdout = GetStdHandle(STD_OUTPUT_HANDLE);
        for buf in bufs {
            WriteFile(
                stdout,
                buf.as_ptr(),
                buf.len() as u32,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            );
        }
    }
}

fn hex(i: u8) -> u8 {
    if i < 0xa {
        i + b'0'
    } else if i < 0x10 {
        i - 0xa + b'a'
    } else {
        panic!("invalid hex");
    }
}

static mut INT_BUF: [u8; 8] = [0; 8];
fn fmt_int(x: u32) -> &'static [u8] {
    unsafe {
        if x == 0 {
            INT_BUF[0] = b'0';
            return &INT_BUF[..1];
        }
        let mut ofs = 0;
        for i in 0..8 {
            let nybble = ((x >> (7 - i) * 4) & 0xf) as u8;
            if ofs == 0 && nybble == 0 {
                continue;
            }
            INT_BUF[ofs] = hex(nybble);
            ofs += 1
        }
        &INT_BUF[..ofs]
    }
}

static mut FLAGS_BUF: [u8; 20] = [0; 20];
fn fmt_flags(flags: u32) -> &'static [u8] {
    unsafe {
        let mut len = 0;
        for (flag, ofs) in [("CF", 0), ("ZF", 6), ("SF", 7), ("DF", 10), ("OF", 11)] {
            if ((flags >> ofs) & 1) != 0 {
                FLAGS_BUF[len..len + 2].copy_from_slice(flag.as_bytes());
                FLAGS_BUF[len + 2] = b' ';
                len += 3;
            }
        }
        if len > 0 {
            &FLAGS_BUF[..(len - 1)]
        } else {
            &FLAGS_BUF[..0]
        }
    }
}

#[inline(always)]
unsafe fn clear_flags() {
    #[cfg(target_arch = "x86")]
    asm!("push 0", "popfd");
}

#[inline(always)]
unsafe fn get_flags() -> u32 {
    #[cfg(target_arch = "x86")]
    {
        let flags: u32;
        asm!("pushfd", "pop {flags}", flags = out(reg) flags);
        flags
    }
    #[cfg(not(target_arch = "x86"))]
    0
}

macro_rules! op {
    ($op:ident, $x:expr, $y:expr) => {
        let x: u32;
        clear_flags();
        #[cfg(target_arch = "x86")]
        asm!(
            concat!("mov {x}, ", $x),
            concat!(stringify!($op), " {x}, ", $y),
            x = out(reg) x,
        );
        #[cfg(not(target_arch = "x86"))]
        {
            x = 0;
        }
        let flags = get_flags();
        print(&[
            concat!(stringify!($op), " ", stringify!($x), ", ", stringify!($y), " => 0x").as_bytes(),
            &fmt_int(x as u32),
            b" ",
            &fmt_flags(flags),
            b"\n",
        ]);
    }
}

macro_rules! run_with_flags {
    ($desc:expr, $asm:tt) => {
        clear_flags();
        #[cfg(target_arch = "x86")]
        let x = $asm;
        #[cfg(not(target_arch = "x86"))]
        let x = 0;
        let flags = get_flags();
        print(&[
            $desc,
            b" => ",
            &fmt_int(x as u32),
            b" ",
            &fmt_flags(flags),
            b"\n",
        ]);
    };
}

macro_rules! adc {
    ($x:expr, $y:expr) => {
        run_with_flags!(concat!("adc (CF=1) ", stringify!($x), ", ", stringify!($y)).as_bytes(), {
            let x: u8;
            asm!(
                "stc",
                concat!("mov {x}, ", $x),
                concat!("adc {x}, ", $y),
                x = out(reg_byte) x
            );
            x
        })
    }
}

unsafe fn add() {
    op!(add, 3, 5);
    op!(add, 3, -3);
    op!(add, 3, -5);

    adc!(0xFF, 0);
    adc!(0xFF, 1);
    adc!(0xFF, 0xFE);
    adc!(0xFF, 0xFF);
}

// rust-analyzer gets confused about this function, so we hide it from rust-analyzer
// following https://github.com/phil-opp/blog_os/issues/1022
#[cfg(not(test))]
#[panic_handler]
unsafe fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    print(&[b"panicked"]);
    windows_sys::Win32::System::Threading::ExitProcess(1);
}

#[no_mangle]
pub unsafe extern "C" fn mainCRTStartup() {
    add();
}
