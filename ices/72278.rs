pub struct RefAny { }

impl RefAny {
    pub fn downcast_mut<'a, U: 'static>(&'a mut self) -> Option<&'a mut U> {
        unsafe { self.downcast_mut_unchecked::<'a, U>() } // <- panic here in lifetime resolution
    }

    unsafe fn downcast_mut_unchecked<'a, U>(&'a mut self) -> Option<&'a mut U> {
        None
    }
}
