#[inline(always)]
fn other_ext() {
    extern "C" {
        pub static _ENARX_SALLYPORT_START: u8;
    }
    unsafe { drop(&_ENARX_SALLYPORT_START) };
}

#[inline(always)]
fn this_ext() {
    extern "C" {
        pub static _ENARX_SALLYPORT_START: u16;
    }
    unsafe { drop(&_ENARX_SALLYPORT_START) };
}

fn main() {
    this_ext();
    other_ext();
}
