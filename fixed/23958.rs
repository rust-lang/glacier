trait MyIntoIterator {
  type IntoIter: Iterator;
  fn into_iter(self) -> Self::IntoIter;
}

// This should be saying `if for any lifetime, a ref to Self is MyIntoIterator`.
trait Iterable where for<'a> &'a Self: MyIntoIterator {
  fn iter<'a>(&'a self) -> <&'a Self as MyIntoIterator>::IntoIter {
    self.into_iter()
  }
}
impl<T> Iterable for T where for<'a> &'a T: MyIntoIterator {}

// Impl MyIntoIterator for &Vec<T> for all lifetimes.
impl<'a, T> MyIntoIterator for &'a Vec<T> {
  type IntoIter = ::std::slice::Iter<'a, T>;
  fn into_iter(self) -> <Self as MyIntoIterator>::IntoIter { self.iter() }
}

// Impl MyIntoIterator for &String for all lifetimes.
impl<'a> MyIntoIterator for &'a String {
  type IntoIter = ::std::str::Chars<'a>;
  fn into_iter(self) -> <Self as MyIntoIterator>::IntoIter { self.chars() }
}

fn iterate<T>(thing: T) where T: Iterable, for<'a> &'a T: MyIntoIterator {
  for x in thing.iter() {
    println!("a thing");
  }
}

fn main() {
  iterate(vec![1,2,3]);
  iterate("abc".to_string());
}
