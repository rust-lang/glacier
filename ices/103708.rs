#![feature(min_specialization)]

trait Dance {}

impl<'a, T> Dance for T {}

impl Dance for bool {}

fn main() {}
