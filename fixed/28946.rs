pub trait Dot<Other=Self> {
    type Output;

    fn dot(self, rhs: Other) -> Self::Output;
}


impl<'a, T, U> Dot<&'a U> for T where T: Dot<U>, U: Copy {
    type Output = T::Output;

    fn dot(self, rhs: &U) -> Self::Output {
        self.dot(*rhs)
    }
}
