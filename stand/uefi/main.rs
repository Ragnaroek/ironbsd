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

type UefiTextReset = extern "C" fn(*const UefiSimpleTextOutputProtocol, extended_verficiation: u8);
type UefiTextString = extern "C" fn(*const UefiSimpleTextOutputProtocol, str: *const u16);

#[repr(C)]
struct UefiSimpleTextOutputProtocol {
    reset: *const UefiTextReset,
    output_string: *const UefiTextString,
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

#[unsafe(no_mangle)]
pub extern "C" fn efi_main(_handle: *const c_void, st_ptr: *const c_void) {
    let st = st_ptr as *const UefiSystemTable;

    let utf16_hello_uefi: [u16; 12] = [
        0x0048, // 'H'
        0x0065, // 'e'
        0x006C, // 'l'
        0x006C, // 'l'
        0x006F, // 'o'
        0x002C, // ','
        0x0020, // ' '
        0x0055, // 'U'
        0x0065, // 'e'
        0x0066, // 'f'
        0x0069, // 'i'
        0x0000,
    ];

    unsafe {
        let out = (*st).con_out;
        let fn_out = *(*out).output_string;
        fn_out(out, utf16_hello_uefi.as_ptr());
    }

    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
