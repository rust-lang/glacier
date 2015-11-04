pub struct Handle {
    pub ptr: *const u32,
}

pub struct ObjectOpResult;
impl ObjectOpResult {
    pub extern "C" fn test(_obj: Handle) {
    }
}
