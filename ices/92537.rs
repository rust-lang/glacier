use std::mem;
use std::marker::PhantomData;

struct LazyStack { }

impl Drop for LazyStack {
    fn drop(&mut self) {
        let g: *mut dyn FnMut(Option<NodeFactory<'_>>) -> Option<VNode<'_>> =
            unsafe {make_fat_ptr()};

        let clos = unsafe { &mut *g };
        clos(None);
    }
}

unsafe fn make_fat_ptr<T: ?Sized>() -> *mut T {
    mem::MaybeUninit::<*mut T>::uninit().assume_init()
}

enum VNode<'src> {
    Fragment(&'src [VNode<'src>]),
}

struct NodeFactory<'a> {
    _x : &'a PhantomData<u32>,
}

fn main() {
    let _x = LazyStack{};
}