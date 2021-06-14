use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Matrix<T>(Vec<T>, usize);

impl<T: Clone> Matrix<T> {
    pub fn new(val: T, len: usize) -> Self {
        Self(vec![val; len * len], len)
    }
}

impl<T> Index<[usize; 2]> for Matrix<T> {
    type Output = T;

    fn index(&self, mut index: [usize; 2]) -> &Self::Output {
        debug_assert!(index[0] < self.1);
        debug_assert!(index[1] < self.1);
        index.sort_unstable();
        unsafe { self.0.get_unchecked(index[0] * self.1 + index[1]) }
    }
}

impl<T> IndexMut<[usize; 2]> for Matrix<T> {
    fn index_mut(&mut self, mut index: [usize; 2]) -> &mut Self::Output {
        debug_assert!(index[0] < self.1);
        debug_assert!(index[1] < self.1);
        index.sort_unstable();
        unsafe { self.0.get_unchecked_mut(index[0] * self.1 + index[1]) }
    }
}

impl<T: Copy> Clone for Matrix<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1)
    }

    fn clone_from(&mut self, source: &Self) {
        self.0.copy_from_slice(&source.0);
    }
}
