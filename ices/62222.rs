#![feature(const_generics)]

use std::mem::MaybeUninit;

#[repr(transparent)]
pub struct Vector<T, const N: usize>([T; N]);

impl<T, const N: usize> Vector<T, {N}> {
    pub fn x(self) -> T {
        let mut head = MaybeUninit::<T>::uninit();
        let mut tail = MaybeUninit::<[T; N - 1]>::uninit();
        let mut from = MaybeUninit::new(self);
        let tailp: *mut T = unsafe { mem::transmute(&mut tail) };
        let fromp: *mut MaybeUninit<T> = unsafe { mem::transmute(&mut from) };
        unsafe {
            head.as_mut_ptr().write(
                fromp
                    .replace(MaybeUninit::uninit())
                    .assume_init()
            );
        }
        for i in 1..N {
            unsafe {
                tailp.add(i - 1).write(
                    fromp
                        .add(i)
                        .replace(MaybeUninit::uninit())
                        .assume_init()
                );
            }
        }
        unsafe { tail.assume_init(); } // Drop the tail
        unsafe { head.assume_init() }
    }
}

fn main() {}
