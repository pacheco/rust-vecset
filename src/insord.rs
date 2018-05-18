use std;
use std::iter::FromIterator;

/// Insertion ordered vec set. Insert as a push and contains as a linear search
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct InsOrdVecSet<T> {
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

    pub fn contains(&self, elem: &T) -> bool {
        if let Some(_) = self.inner.iter().position(|it| it == elem) {
            true
        } else {
            false
        }
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        if self.is_empty() || other.is_empty() {
            return true ;
        }
        for e in self.inner.iter() {
            if other.contains(e) {
                return false;
            }
        }
        return true;
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

#[cfg(test)]
mod tests {
    use ::InsOrdVecSet;

    #[test]
    fn is_disjoint() {
        let a: InsOrdVecSet<_> = [1,2,3,4,5].iter().collect();
        let b: InsOrdVecSet<_> = [3,4,5,6,7].iter().collect();
        assert!(!a.is_disjoint(&b));

        let a: InsOrdVecSet<_> = [1,2,3,4,5].iter().collect();
        let b: InsOrdVecSet<_> = [5].iter().collect();
        assert!(!a.is_disjoint(&b));

        let a: InsOrdVecSet<_> = [1,2,3,4,5].iter().collect();
        let b: InsOrdVecSet<_> = [10,6,0].iter().collect();
        assert!(a.is_disjoint(&b));

        let a: InsOrdVecSet<_> = [1,2,3,4,5].iter().collect();
        let b: InsOrdVecSet<_> = [].iter().collect();
        assert!(a.is_disjoint(&b));

        let a: InsOrdVecSet<_> = [].iter().collect();
        let b: InsOrdVecSet<&usize> = [1].iter().collect();
        assert!(a.is_disjoint(&b));

        let a: InsOrdVecSet<&()> = [].iter().collect();
        let b: InsOrdVecSet<&()> = [].iter().collect();
        assert!(a.is_disjoint(&b));
    }

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
}
