pub struct EntriesBuffer(Box<[[u8; HashesEntry::LEN]; 5]>);

impl EntriesBuffer {
    pub fn iter_child_buffers(&mut self) -> impl Iterator<Item = &mut [u8; HashesEntry::LEN]> {
        self.0.iter_mut()
    }
}

pub struct HashesEntry<'a>(&'a [u8]);

impl HashesEntry<'_> {
    pub const LEN: usize = 1;
}
