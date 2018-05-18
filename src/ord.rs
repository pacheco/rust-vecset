use std;
use std::cmp::Ordering;
use std::iter::FromIterator;

/// Ordered vec set
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct OrdVecSet<T> {
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

    pub fn min(&self) -> Option<&T> {
        self.inner.first()
    }

    pub fn max(&self) -> Option<&T> {
        self.inner.last()
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

    pub fn is_disjoint(&self, other: &Self) -> bool {
        let mut i = 0;
        let mut j = 0;

        'outer: while let Some(e) = self.inner.get(i) {
            while let Some(o) = other.inner.get(j) {
                match e.cmp(o) {
                    Ordering::Equal => { return false; }
                    Ordering::Less => { i += 1; continue 'outer; } // advance self
                    Ordering::Greater => { j += 1; continue; } // advance other
                }
            }
            break;
        }

        return true;
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

#[cfg(test)]
mod tests {
    use ::OrdVecSet;

    #[test]
    fn is_disjoint() {
        let a: OrdVecSet<_> = [1,2,3,4,5].iter().collect();
        let b: OrdVecSet<_> = [3,4,5,6,7].iter().collect();
        assert!(!a.is_disjoint(&b));

        let a: OrdVecSet<_> = [1,2,3,4,5].iter().collect();
        let b: OrdVecSet<_> = [5].iter().collect();
        assert!(!a.is_disjoint(&b));

        let a: OrdVecSet<_> = [1,2,3,4,5].iter().collect();
        let b: OrdVecSet<_> = [10,6,0].iter().collect();
        assert!(a.is_disjoint(&b));

        let a: OrdVecSet<_> = [1,2,3,4,5].iter().collect();
        let b: OrdVecSet<_> = [].iter().collect();
        assert!(a.is_disjoint(&b));

        let a: OrdVecSet<_> = [].iter().collect();
        let b: OrdVecSet<&usize> = [1].iter().collect();
        assert!(a.is_disjoint(&b));

        let a: OrdVecSet<&()> = [].iter().collect();
        let b: OrdVecSet<&()> = [].iter().collect();
        assert!(a.is_disjoint(&b));
    }

    #[test]
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
