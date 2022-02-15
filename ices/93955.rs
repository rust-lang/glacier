use std::ops::Add;
struct Point<T> {
    x: T,
    y: T,
}

impl<'a, T> Add for &'a Point<T>
where
    for<'x> &'x T: Add<Output = T>,
{
    type Output = Point<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: &self.x + &rhs.x,
            y: &self.y + &rhs.y,
        }
    }
}

fn add<T, U>(x: T, y: T) -> T
where
    T: From<U>,
    for<'x> &'x T: Add<Output = U>,
{
    T::from(&x + &y)
}

fn main() {
    let x = 2;
    let y = 3;
    //let a = add(x, y); // E0275
    let b = add::<i32>(x, y); // ICE
    //let c = add::<i32, i32>(x, y); // works
}
