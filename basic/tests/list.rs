use basic::List;

#[cfg(test)]
mod list_tests {

    use super::*;

    #[test]
    fn is_empty() {
        let mut list = List::<i32>::new();
        assert!(list.is_empty());
        list.add_front(1);
        assert!(!list.is_empty());
        list.add_front(2);
        assert!(!list.is_empty());
        assert_eq!(list.rm_front().unwrap(), 2);
        assert!(!list.is_empty());
        assert_eq!(list.rm_front().unwrap(), 1);
        assert!(list.is_empty());
    }

    #[test]
    fn size() {
        let mut list = List::<i32>::new();
        assert_eq!(list.size(), 0);
        list.add_front(1);
        assert_eq!(list.size(), 1);
        list.add_front(3);
        assert_eq!(list.size(), 2);
        list.add_after(2, &3);
        assert_eq!(list.size(), 3);
        list.add_tail(0);
        assert_eq!(list.size(), 4);
        assert_eq!(list.rm_front().unwrap(), 3);
        assert_eq!(list.size(), 3);
        assert!(!list.is_empty());
        assert_eq!(list.rm_front().unwrap(), 2);
        assert_eq!(list.size(), 2);
        assert!(!list.is_empty());
        assert_eq!(list.rm_front().unwrap(), 1);
        assert_eq!(list.size(), 1);
        assert!(!list.is_empty());
        assert_eq!(list.rm_front().unwrap(), 0);
        assert_eq!(list.size(), 0);
        assert!(list.is_empty());
    }
}
