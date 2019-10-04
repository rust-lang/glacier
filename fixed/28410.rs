
extern crate x86;

use core::{ptr, intrinsics};
use self::x86::io::{inb, outb};

/***** Serial I/O code *****/
const COM1: u16              = 0x3f8;

const COM_RX: u16            = 0;    // In:  Receive buffer (DLAB=0)
const COM_TX: u16            = 0;    // Out: Transmit buffer (DLAB=0)
const COM_DLL: u16           = 1;    // Out: Divisor Latch Low (DLAB=1)
const COM_DLM: u16           = 1;    // Out: Divisor Latch High (DLAB=1)
const COM_IER: u16           = 1;    // Out: Interrupt Enable Register
const COM_IER_RDI: u8        = 0x01; //   Enable receiver data interrupt
const COM_IIR: u16           = 2;    // In: Interrupt ID Register
const COM_FCR: u16           = 2;    // Out: FIFO Control Register
const COM_LCR: u16           = 3;    // Out: Line Control Register
const COM_LCR_DLAB: u8       = 0x80; //   Divisor latch access bit
const COM_LCR_WLEN8: u8      = 0x03; //   Wordlength: 8 bits
const COM_MCR: u16           = 4;    // Out: Modem Control Register
const COM_MCR_RTS: u8        = 0x02; // RTS complement
const COM_MCR_DTR: u8        = 0x01; // DTR complement
const COM_MCR_OUT2: u8       = 0x08; // Out2 complement
const COM_LSR: u16           = 5;    // In: Line Status Register
const COM_LSR_DATA: u8       = 0x01; //   Data available
const COM_LSR_TXRDY: u8      = 0x20; //   Transmit buffer avail
const COM_LSR_TSRE: u8       = 0x40; //   Transmitter off

static mut serial_exists: bool = false;

const MONO_BASE: u16         = 0x3b4u16;
const MONO_BUF: u32          = 0xb0000;
const CGA_BASE: u16          = 0x3D4u16;
const CGA_BUF: u32           = 0xb8000;

const CRT_ROWS: u32          = 25;
const CRT_COLS: u32          = 80;
const CRT_SIZE: u32          = CRT_ROWS * CRT_COLS;

static mut crt_buf: *mut u16 = 0 as *mut u16;
static mut crt_pos: u16      = 0;

pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}

pub fn clear_screen(background: u16) {
    for i in 0..80*25 {
        unsafe {
            *((0xb8000 + i * 2) as *mut u16) = (background as u16) << 12;
        }
    }
}

fn cga_init() {
    unsafe {
        crt_buf = CGA_BUF as *mut u16;
        let was: u16 = intrinsics::volatile_load(crt_buf);
        intrinsics::volatile_store(crt_buf, 0xa55a);
        let addr_6845: u16 =
            if intrinsics::volatile_load(crt_buf as *const u16) != 0xa55a {
                crt_buf = MONO_BUF as *mut u16;
                MONO_BASE
            } else {
                intrinsics::volatile_store(crt_buf, was);
                CGA_BASE
            };
        outb(addr_6845, 14);
        crt_pos  = (inb(addr_6845 + 1) as u16) << 8;
        outb(addr_6845, 15);
        crt_pos |= inb(addr_6845 + 1) as u16;
    }
}

fn serial_init() {
    unsafe {
        // Turn off the FIFO
        outb(COM1+COM_FCR, 0);

        // Set speed; requires DLAB latch
        outb(COM1+COM_LCR, COM_LCR_DLAB);
        outb(COM1+COM_DLL, (115200 / 9600) as u8);
        outb(COM1+COM_DLM, 0);
        
        // 8 data bits, 1 stop bit, parity off; turn off DLAB latch
        outb(COM1+COM_LCR, COM_LCR_WLEN8 & (!COM_LCR_DLAB));
        
        // No modem controls
        outb(COM1+COM_MCR, 0);
        // Enable rcv interrupts
        outb(COM1+COM_IER, COM_IER_RDI);
        
        // Clear any preexisting overrun indications and interrupts
        // Serial port doesn't exist if COM_LSR returns 0xFF
        serial_exists = inb(COM1+COM_LSR) != 0xff;
        
        inb(COM1+COM_IIR);
        inb(COM1+COM_RX);
    }
}

pub fn console_init() {
    cga_init();
    serial_init();
}
