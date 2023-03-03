#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub enum Restriction<const V: bool = true> {}
pub trait Valid {}

impl Valid for Restriction<true> {}

/**
    a circular buffer
*/
pub struct CircularBuffer<T, const S: usize> {
    buffer: Vec<T>,
    index: usize,
}

impl<T, const S: usize> CircularBuffer<T, { S }>
where
    Restriction<{ S > 0 }>: Valid,
{
    /**
        create a new [`CircularBuffer`] instance
    */
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(S),
            index: S - 1,
        }
    }

    /**
        push a value onto the buffer
    */
    pub fn push(&mut self, value: T) {
        self.index = self.index.checked_add(1).unwrap_or(0) % S;
        match self.count() < self.capacity() {
            true => self.buffer.insert(self.index, value),
            false => self.buffer[self.index] = value,
        }
    }

    /**
        push a value onto the buffer
    */
    pub fn pop(&mut self) -> Result<T, CircularBufferError> {
        if self.count() == 0 {
            Err(CircularBufferError::Underflow)?
        }

        let old = self.buffer.remove(self.index);
        self.index = self.index.checked_sub(1).unwrap_or(S - 1);
        Ok(old)
    }
}

impl<T, const S: usize> CircularBuffer<T, { S }> {
    /**
        returns the current count of the buffer
    */
    pub fn count(&self) -> usize {
        self.buffer.len()
    }

    /**
        returns the max capacity of the buffer
    */
    pub const fn capacity(&self) -> usize {
        S
    }
}

impl<T, const S: usize> CircularBuffer<T, { S }>
where
    T: Clone,
{
    /**
        take the values in a vec, leaving an empty buffer
    */
    pub fn take(&mut self) -> Vec<T> {
        let mut buf = Vec::with_capacity(self.buffer.len());
        buf.copy_from_slice(self.buffer[..]);
        self.index = S - 1;
        buf
    }

    /**
        returns true if the buffer is empty
    */
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

#[derive(Debug, PartialEq)]
pub enum CircularBufferError {
    /// attempted to pop when buffer is empty
    Underflow,
}

fn main() {}
