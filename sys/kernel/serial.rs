/// A minimal wrapper for x86 port I/O (stable Rust)
pub struct Port<T> {
    port: u16,
    _marker: core::marker::PhantomData<T>,
}

impl Port<u8> {
    pub const unsafe fn new(port: u16) -> Self {
        Self {
            port,
            _marker: core::marker::PhantomData,
        }
    }

    pub unsafe fn write(&mut self, value: u8) {
        unsafe {
            // SAFETY: This is unsafe because it directly uses x86 port I/O.
            // Only use this on x86 targets.
            core::arch::asm!(
                "out dx, al",
                in("dx") self.port,
                in("al") value,
                options(nostack, preserves_flags)
            );
        }
    }

    pub unsafe fn read(&mut self) -> u8 {
        unsafe {
            let value: u8;
            // SAFETY: This is unsafe because it directly uses x86 port I/O.
            core::arch::asm!(
                "in al, dx",
                in("dx") self.port,
                out("al") value,
                options(nostack, preserves_flags)
            );
            value
        }
    }
}

pub unsafe fn init_serial() {
    unsafe {
        let mut port = Port::<u8>::new(0x3F8 + 3);
        port.write(0x80); // Enable DLAB

        let mut port0 = Port::<u8>::new(0x3F8 + 0);
        port0.write(0x01); // Low byte of divisor (1)

        let mut port1 = Port::<u8>::new(0x3F8 + 1);
        port1.write(0x00); // High byte of divisor (0)

        let mut port3 = Port::<u8>::new(0x3F8 + 3);
        port3.write(0x03); // 8N1

        let mut port2 = Port::<u8>::new(0x3F8 + 2);
        port2.write(0xC7); // Enable FIFO
    }
}

/// Check if the transmit buffer is empty
unsafe fn is_transmit_empty() -> bool {
    unsafe {
        let mut port = Port::<u8>::new(0x3F8 + 5);
        port.read() & 0x20 != 0
    }
}

/// Write a byte to COM1
unsafe fn serial_write_byte(byte: u8) {
    unsafe {
        let mut port = Port::<u8>::new(0x3F8);
        while !is_transmit_empty() {}
        port.write(byte);
    }
}

/// Write a string to COM1
pub unsafe fn serial_write_str(s: &str) {
    for byte in s.bytes() {
        unsafe { serial_write_byte(byte) };
    }
}
