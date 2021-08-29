trait Lf<'a> { type Type; }

struct F32;
impl<'a> Lf<'a> for F32 { type Type = f32; }

struct RefI32;
impl<'a> Lf<'a> for RefI32 { type Type = &'a i32; }

fn add_closure<T: for<'b> Lf<'b> + 'static>(v: &mut Vec<Box<for<'a> FnMut(<T as Lf<'a>>::Type)>>) {
    v.push(Box::new(move |_| {
    }));
}

fn main() {
    // bug? compiler can't seem to deduce <F32 as Lf<'a>>::Type == f32 nor <RefI32 as Lf<'a>>::Type == &'a i32
    let mut v1 = Vec::<Box<for<'a> FnMut(<F32 as Lf<'a>>::Type)>>::new();
    let mut v2 = Vec::<Box<for<'a> FnMut(<RefI32 as Lf<'a>>::Type)>>::new();
    add_closure::<F32>(&mut v1);
    add_closure::<RefI32>(&mut v2);
}
