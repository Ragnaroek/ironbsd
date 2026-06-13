#![no_std]
#![no_main]

use core::ffi::c_void;

#[repr(C)]
struct UefiTableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc_32: u32,
    reserved: u32,
}

#[repr(C)]
struct UefiSimpleTextOutputProtocol {
    reset: unsafe extern "win64" fn(*const Self, extended_verificiation: u8) -> usize,
    output_string: unsafe extern "win64" fn(this: *const Self, string: *const u16) -> usize,
}

#[repr(C)]
struct UefiSystemTable {
    hdr: UefiTableHeader,
    firmware_vendor: *const u16,
    firmware_revision: u32,
    console_in_handle: *const c_void,
    con_in: *const c_void,
    console_out_handle: *const c_void,
    con_out: *const UefiSimpleTextOutputProtocol,
}

const MAX_OUTPUT: usize = 101;

/// A tiny abstraction over raw UEFI. Only what is needed
/// for the boot procress.
struct Uefi {
    output_buffer: [u16; MAX_OUTPUT],
    st: *const UefiSystemTable,
}

impl Uefi {
    fn init(st: *const UefiSystemTable) -> Self {
        Uefi {
            output_buffer: [0; MAX_OUTPUT],
            st,
        }
    }

    /// ASCII only supported. Max 100 characters.
    fn output_string(&mut self, str_in: &str) {
        let str = if str_in.len() >= MAX_OUTPUT {
            &str_in[0..(MAX_OUTPUT - 1)]
        } else {
            str_in
        };

        for (i, &byte) in str.as_bytes().iter().enumerate() {
            self.output_buffer[i] = byte as u16;
        }
        self.output_buffer[str.len()] = 0;
        unsafe {
            let out = (*self.st).con_out;
            let fn_out = (*out).output_string;
            fn_out(out, self.output_buffer.as_ptr());
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn efi_main(_handle: *const c_void, st: *const UefiSystemTable) {
    let mut uefi = Uefi::init(st);
    uefi.output_string("UEFI Firmware v1: Starting boot process...\n");
    // TODO Load the kernel image with uefi
    uefi.output_string("Kernel loaded and starting\n");
    // TODO jump to kernel
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
