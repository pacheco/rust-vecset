use std;
use std::iter::FromIterator;

/// Ordered vec set
#[derive(Debug, Clone)]
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

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
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

    pub fn inner_mut(&mut self) -> &mut Vec<T> {
        &mut self.inner
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

    pub fn remove(&mut self, elem: &T) -> bool {
        if let Ok(pos) = self.inner.binary_search(&elem) {
            self.inner.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn contains(&self, elem: &T) -> bool {
        match self.inner.binary_search(&elem) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}


impl<I: Ord> FromIterator<I> for OrdVecSet<I> {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = I>
    {
        let mut inner: Vec<_> = iter.into_iter().collect();
        inner.sort();
        OrdVecSet {
            inner,
        }
    }
}
