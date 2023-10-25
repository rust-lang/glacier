#![feature(generic_const_exprs)]

pub async fn something(path: &[[usize; N_ISLANDS]; PrivateStruct]) -> usize {
    match path {
        [] | _ => 0,
    }
}

pub fn main() {}

