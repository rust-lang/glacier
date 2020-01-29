trait Trait { type Resources: Resources; }
impl Trait for () {
    type Resources = usize;
}


trait ResourceFamily<'a> { type Output; }

struct UsizeResourceFamily;
impl<'a> ResourceFamily<'a> for UsizeResourceFamily {
    type Output = &'a usize;
}

trait Resources { type Family: for<'a> ResourceFamily<'a>; }
impl Resources for usize {
    type Family = UsizeResourceFamily;
}

fn test<T: Trait>() {
    let _: Box<dyn Fn(&mut <<<T as Trait>::Resources as Resources>::Family as ResourceFamily>::Output)> = Box::new(|_| {});
}

fn main() {
    test::<()>();
}
