use std::ops::Sub;

trait Vector2 {
    type ScalarType;
    fn from_values(x: Self::ScalarType, y: Self::ScalarType) -> Self
    where Self: Sized;
    fn x(&self) -> Self::ScalarType;
    fn y(&self) -> Self::ScalarType;
}

impl<T> Sub for dyn Vector2<ScalarType=T>
where T: Sub<Output=T>,
(dyn Vector2<ScalarType=T>): Sized{
    type Output = dyn Vector2<ScalarType=T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_values(self.x()-rhs.x(), self.y() - rhs.y())
    }
}

fn main() {
    
}
