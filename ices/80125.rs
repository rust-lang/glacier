type ExternCallback = extern "C" fn(*const u8, u32, str);

pub struct Struct(ExternCallback);

#[no_mangle]
pub extern "C" fn register_something(bind: ExternCallback) -> Struct {
    Struct(bind)
}
