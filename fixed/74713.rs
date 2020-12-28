fn bug<'a>()
where
    [(); {
        let _: &'a ();
    }]: ,
{
}

fn main() {}
