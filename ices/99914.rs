struct Error;

fn foo() {
    let initial_exchange: Result<usize, Error> = todo!();
    initial_exchange.and_then(|_|
        serve_udp_tunnel()
    ).await;
}

async fn serve_udp_tunnel() {}
