use std;
use std::iter::FromIterator;

/// Insertion ordered vec set. Insert as a push and contains as a linear search
#[derive(Debug)]
pub struct InsOrdVecSet<T: Eq> {
    inner: Vec<T>,
}

impl<T: Eq> InsOrdVecSet<T> {
    pub fn new() -> Self {
        InsOrdVecSet::with_capacity(0)
    }

    pub fn with_capacity(cap: usize) -> Self {
        InsOrdVecSet {
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

    pub fn inner_mut(&mut self) -> &mut Vec<T> {
        &mut self.inner
    }

    pub fn into_inner(self) -> Vec<T> {
        self.inner
    }

    pub fn insert(&mut self, elem: T) -> bool {
        if let Some(_) = self.inner.iter().position(|it| it == &elem) {
            false
        } else {
            self.inner.push(elem);
            true
        }
    }

    pub fn remove(&mut self, elem: &T) -> bool {
        if let Some(pos) = self.inner.iter().position(|it| &it == &elem) {
            self.inner.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn contains(&mut self, elem: &T) -> bool {
        if let Some(_) = self.inner.iter().position(|it| it == elem) {
            true
        } else {
            false
        }
    }
}

impl<I: Eq> FromIterator<I> for InsOrdVecSet<I> {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = I>
    {
        InsOrdVecSet {
            inner: iter.into_iter().collect(),
        }
    }
}
