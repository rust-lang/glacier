#!/bin/sh

rustc --edition=2021 - << EOF

pub enum Request {
    Resolve {
        url: String,
    },
}

pub async fn handle_event(
    event: Request,
)  {
    async move {
        let Request::Resolve { url } = event;
    }.await;
}

pub fn main() {}

EOF
