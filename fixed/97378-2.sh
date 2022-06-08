#!/bin/sh

rustc --edition=2021 - << EOF

pub enum Request {
    Resolve { url: String },
}

pub fn handle_event(event: Request) {
    (move || {
        let Request::Resolve { url: _url } = event;
    })();
}

pub fn main() {}

EOF
