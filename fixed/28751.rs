struct DataType {
    data: *mut (),
}

unsafe impl Sync for DataType {}

static mut EXTRA_DATA: () = ();

static DATA : DataType = DataType {
    data: unsafe { &mut EXTRA_DATA }
};

fn main() {
}
