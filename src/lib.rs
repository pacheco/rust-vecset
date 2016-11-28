// pub struct SortedVecSet<T: Ord> {
//     inner: Vec<T>,
// }

// impl<T: Ord> SortedVecSet<T> {
// }

// Insertion ordered vec set ------------------------------------------------------

/// Insertion order small vec set
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

    pub fn contains(&mut self, elem: &T) -> bool {
        if let Some(_) = self.inner.iter().position(|it| it == elem) {
            true
        } else {
            false
        }
    }
}

// Ordered vec set ------------------------------------------------------
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


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insertion_order_vecset() {
        let mut set = InsOrdVecSet::new();
        assert!(set.insert(1));
        assert!(!set.insert(1));
        assert_eq!(set.len(), 1);

        assert!(set.insert(3));
        assert!(!set.insert(3));
        assert_eq!(set.len(), 2);

        assert!(set.insert(2));
        assert!(!set.insert(2));
        assert_eq!(set.len(), 3);

        assert_eq!(&set.inner()[..], &[1,3,2]);
    }

    fn test_ordered_vecset() {
        let mut set = OrdVecSet::new();
        assert!(set.insert(1));
        assert!(!set.insert(1));
        assert_eq!(set.len(), 1);

        assert!(set.insert(3));
        assert!(!set.insert(3));
        assert_eq!(set.len(), 2);

        assert!(set.insert(2));
        assert!(!set.insert(2));
        assert_eq!(set.len(), 3);

        assert_eq!(&set.inner()[..], &[1,2,3]);
    }
}
