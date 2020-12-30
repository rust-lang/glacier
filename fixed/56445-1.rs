struct OnDiskDirEntry<'a> { _s: &'a usize }

impl<'a> OnDiskDirEntry<'a> {
    const LFN_FRAGMENT_LEN: usize = 2;

    fn lfn_contents(&self) -> [char; Self::LFN_FRAGMENT_LEN] { loop { } }
}

fn main() {}
