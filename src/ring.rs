use std::collections::VecDeque;
use std::ops::{Index, IndexMut};

pub struct RingBuffer<T> {
    data: VecDeque<T>,
    // Abstract index of data[0] in infinitely sized queue
    offset: usize,
}

impl<T> RingBuffer<T> {
    pub fn new() -> Self {
        RingBuffer {
            data: VecDeque::new(),
            offset: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.push_back(value);
    }

    pub fn advance_right(&mut self)
    where
        T: Default,
    {
        self.data.push_back(T::default());
    }

    pub fn advance_left(&mut self) {
        self.data.pop_front().unwrap();
        self.offset += 1;
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl<T> Index<usize> for RingBuffer<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index.checked_sub(self.offset).unwrap()]
    }
}

impl<T> IndexMut<usize> for RingBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index.checked_sub(self.offset).unwrap()]
    }
}