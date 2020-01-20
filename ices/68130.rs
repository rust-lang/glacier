#![feature(const_generics)]

pub type MatrixArray<T, const N : usize, const M : usize> = [T; N * M];

pub struct Matrix<T, const N : usize, const M : usize>
    where T : Copy + Sized
{
    store : MatrixArray<T, N, M>,
}

impl<T, const N : usize, const M : usize> Default for Matrix<T, N, M>
    where T : Copy + Sized + Default
{
    fn default() -> Self {
        let zero_comps = unsafe {
            let mut comps = std::mem::MaybeUninit::< MatrixArray<T, N, M> >::uninit().assume_init();
            for comp in comps.iter_mut() {
                *comp = T::default();
            }
            comps
        };
        Self {
            store : zero_comps
        }
    }
}

fn main() {}
