type ExternCallback = extern "C" fn(*const u8, u32, str);

pub struct Client {
    // ...
    conn_tx_bind: ExternCallback,
    // ...
}

#[no_mangle]
pub extern fn register_something(bind: ExternCallback) -> Client {
    Client{
        conn_tx_bind: bind
    }
}

fn main() {}
