use std;

/// Ordered vec set
pub struct OrdVecSet<T: Ord> {
    inner: Vec<T>,
}

impl<T: Ord> OrdVecSet<T> {
    pub fn new() -> Self {
        OrdVecSet::with_capacity(0)
    }

    pub fn with_capacity(cap: usize) -> Self {
        OrdVecSet {
            inner: Vec::with_capacity(cap),
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.inner.iter_mut()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<T> {
        self.inner.into_iter()
    }

    pub fn inner(&self) -> &Vec<T> {
        &self.inner
    }

    pub fn into_inner(self) -> Vec<T> {
        self.inner
    }

    pub fn insert(&mut self, elem: T) -> bool {
        match self.inner.binary_search(&elem) {
            Ok(_) => false,
            Err(pos) => {
                self.inner.insert(pos, elem);
                true
            }
        }
    }

    pub fn contains(&mut self, elem: &T) -> bool {
        match self.inner.binary_search(&elem) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
