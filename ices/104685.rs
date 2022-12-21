#![feature(extern_types)]

extern "C" {
    pub type ExternType;
}

extern "C" {
    pub static EXTERN: ExternType;
}

pub static EMPTY: () = unsafe {
    &EXTERN;
};

fn main() {}
