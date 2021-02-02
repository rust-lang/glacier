struct InterruptSectorManager {
    field: &'static (),
}

static FOO: [InterruptSectorManager; 2] = [InterruptSectorManager { field: &() }; 2];

fn main() {}
