mod insord;
mod ord;

pub use insord::*;
pub use ord::*;

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
