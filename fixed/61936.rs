#![feature(const_generics)]

trait SliceExt<T: Clone> {
    fn array_windows<'a, const N: usize>(&'a self) -> ArrayWindows<'a, T, {N}>;
}

impl <T: Clone> SliceExt<T> for [T] {
   fn array_windows<'a, const N: usize>(&'a self) -> ArrayWindows<'a, T, {N}> {
       ArrayWindows{ idx: 0, slice: &self }
   } 
}

struct ArrayWindows<'a, T, const N: usize> {
    slice: &'a [T],
    idx: usize,
}

impl <'a, T: Clone, const N: usize> Iterator for ArrayWindows<'a, T, {N}> {
    type Item = [T; N];
    fn next(&mut self) -> Option<Self::Item> {
        let mut res = unsafe{ std::mem::zeroed() };
        let mut ptr = &mut res as *mut [T; N] as *mut T;
        
        for i in 0..N {
            match self.slice[i..].get(i) {
                None => return None,
                Some(elem) => unsafe { std::ptr::write_volatile(ptr, elem.clone())}, 
            };
            ptr = ptr.wrapping_add(1);
            self.idx += 1;
        }
        
        Some(res)
    }
}

const FOUR: usize = 4;

fn main() {
    let v = vec![100; 0usize];
    
    for array in v.as_slice().array_windows::<{FOUR}>() {
       // println!("{:?}", array);
    }
}
