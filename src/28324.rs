extern {
    static error_message_count: u32; // anything will do
}
pub static BAZ: u32 = *&error_message_count;
fn main() {}
