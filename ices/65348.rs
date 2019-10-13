struct ArrayType<T>(T);

impl <T> ArrayType<T> {
    const ARRAY: [T; 0] = [];
}

pub const fn bug<T>() ->  &'static T {
    &ArrayType::<T>::ARRAY[0]
}

fn main() {
    bug::<i32>();
}
