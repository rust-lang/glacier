#![feature(rustc_attrs)]

extern "C" {
    #[rustc_args_required_const(0, 1)]
    pub fn print_raw(data: *const u8, len: usize);
}

pub enum Stmt {
    Print {},
    Let {},
    Loop {},
}

pub fn interpret() {
    match (Stmt::Print {}) {
        Stmt::Let {} => {
            [()].iter().find(|v| *v == &());
        }

        Stmt::Print {} => {
            print_raw(b"hello".as_ptr(), 5);
        }

        Stmt::Loop {} => {}
    }
}

fn main() {}
