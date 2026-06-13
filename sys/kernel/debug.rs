use core::fmt::Write;

fn ptr_to_str(ptr: *const c_void, buffer: &'static mut [u8; 20]) -> &'static str {
    unsafe {
        let mut writer = BufferWriter {
            buffer: buffer,
            pos: 0,
        };
        write!(&mut writer, "0x{:x}", ptr as usize).unwrap();
        core::str::from_utf8_unchecked(buffer)
    }
}

fn usize_to_str(n: usize, buffer: &'static mut [u8; 20]) -> &'static str {
    *buffer = [0; 20];
    unsafe {
        let mut writer = BufferWriter {
            buffer: buffer,
            pos: 0,
        };
        write!(&mut writer, "{}", n).unwrap();
        core::str::from_utf8_unchecked(buffer)
    }
}

struct BufferWriter<'a> {
    pub buffer: &'a mut [u8; 20],
    pos: usize,
}

impl<'a> Write for BufferWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        if self.pos + bytes.len() > self.buffer.len() {
            return Err(core::fmt::Error);
        }
        self.buffer[self.pos..self.pos + bytes.len()].copy_from_slice(bytes);
        self.pos += bytes.len();
        Ok(())
    }
}
