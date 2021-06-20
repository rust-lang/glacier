#!/bin/bash

rustc --target aarch64-apple-darwin - <<'EOF'
use std::{fs::File, io::Read, env::args};

fn main() -> ! {
    let args:Vec<String> = args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments!")
    }
    // FIXME: Code makes bad assumption that first arg
    // will always be the file.
    let mut f = File::open(&args[1]).unwrap();
    // String gets written to by .read_to_string()
    let mut file = String::new();
    // TODO: Implement some proper error handling
    f.read_to_string(&mut file).unwrap();
    // Drop file handle. From now on, it's unnecessary.
    drop(f);
    // Make 'file' immutable (There HAS to be a better way of doing this!)
    let file = file;
    println!("{}", file);
    todo!("Do something with file")
}
EOF
