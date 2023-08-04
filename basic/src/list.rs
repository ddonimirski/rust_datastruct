///
/// simple signle link list
///
/// expected API and performance:
///
///   fn is_empty() -> bool                 | O(1) |
///   fn push_front(item:T)                  | O(1) |
///   fn rm_front(item:T) -> Option<T>      | O(1) |
///   fn rm_first(item:T) -> Option<T>      | O(N) | todo
///   fn rm_all(item:T) -> Option<Vec<T>>   | O(N) | todo ?
///   fn head() -> Option<&mut T>           | O(1) |
///   fn tail() -> Option<&mut T>           | O(1) |
///   fn size() -> usize                    | O(1) |
///   fn contains(item:T) -> bool           | O(N) |
///   fn add_after(item:T, elem:&T)         | O(N) |
///   fn push_back(item:T)                   | O(1) |
///


// #![feature(trait_alias)]
// trait TConcept = std::fmt::Debug + std::cmp::PartialEq;

type Pointer<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T:std::fmt::Debug + std::cmp::PartialEq> {
    value: T,
    next: Pointer<T>
}

impl<T:std::fmt::Debug + std::cmp::PartialEq> Node<T> {
    fn new(value:T) -> Self {
        Self {
            value,
            next: None,
        }
    }

    fn create(value:T, next: Pointer<T>) -> Self {
        Self {
            value,
            next,
        }
    }
}

#[derive(Debug)]
pub struct ListIterator<'a, T: std::fmt::Debug + std::cmp::PartialEq> {
    next_node: Option<&'a Node<T>>
}

/// keeps single linked nodes
/// 
#[derive(Debug)]
pub struct List<T:std::fmt::Debug + std::cmp::PartialEq> {
    head: Pointer<T>,
    tail: *mut Node<T>,
    size: usize,
}



impl<T:std::fmt::Debug + std::cmp::PartialEq> List<T> {

    fn set_tail(&mut self, node: &mut Pointer<T>) {
            self.tail = match node {
                Some(node) => &mut ** node as *mut Node<T>,
                None => std::ptr::null_mut(),
            }
    }

    /// retuns head iterator
    /// 
    pub fn iter(&self) -> ListIterator<'_, T> {
        ListIterator{ next_node: self.head.as_deref() }
    }

    /// create new empty list
    /// 
    /// # Example
    /// ``` 
    /// use basic::List;
    /// 
    /// let list = List::<i32>::new();
    /// ```
    /// 
    pub fn new() -> Self {
        Self {
            head: None,
            tail: std::ptr::null_mut(),
            size: 0,
        }
    }

    /// cheke if the list is empty
    /// 
    /// # Example
    /// ```
    /// use basic::List;
    /// 
    /// let list = List::<i32>::new();
    /// let empty = list.is_empty();
    /// assert_eq!(true, empty);
    /// ```
    /// 
    pub fn is_empty(&self) -> bool {
        assert!((self.head.is_none() && self.tail.is_null() && self.size == 0)
                || (!self.head.is_none() && !self.tail.is_null()));
        self.head.is_none()
    }

    /// add item at the front of the list
    /// 
    /// # Example
    /// ```
    /// use basic::List;
    /// 
    /// let mut list = List::<i32>::new();
    /// list.push_front(2);
    /// assert_eq!(false, list.is_empty());
    /// ```
    pub fn push_front(&mut self, item:T) {
        self.head = Some(Box::new(Node::create(item, self.head.take())));

        if self.tail.is_null() {
            self.tail = match self.head.as_mut() {
                Some(node) => &mut ** node as *mut Node<T>,
                None => std::ptr::null_mut(),
            }
        }

        self.size += 1;
    }

    /// rm item from the front (head) of the list
    /// retruns removed item if any
    /// 
    /// # Example
    /// ```
    /// use basic::List;
    /// 
    /// let mut list = List::<i32>::new();
    /// list.push_front(2);
    /// list.push_front(1);
    /// assert_eq!(list.rm_front().unwrap(), 1);
    /// assert_eq!(*list.head().unwrap(), 2);
    /// ```
    pub fn rm_front(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(old_head) => {
                self.head = old_head.next;
                self.size -= 1;
                if self.head.as_ref().is_none() {
                    self.tail = std::ptr::null_mut();
                    assert_eq!(self.size, 0);
                }
                
                // return the removed value
                Some(old_head.value)
            }
        }
    }

    /// rm first epirance
    pub fn rm_first(&mut self, elem: &T) -> Option<T> {
        todo!()
    }

    /// remove all apirance of the item in the list
    /// returns list of removed items ?
    /// 
    pub fn rm_all(&mut self, elem: &T) -> Option<Vec<T>> {
        todo!()
    }

    /// counts elemetes 
    /// 
    /// # Example
    /// ```
    /// use basic::List;
    /// 
    /// let mut list = List::<i32>::new();
    /// list.push_front(1);
    /// list.push_front(1);
    /// list.push_front(1);
    /// list.push_front(1);
    /// assert_eq!(list.count(&1), 4 as usize);
    /// ```
    /// 
    pub fn count(&self, elem: &T) -> usize {

        /*
        let mut num = 0 as usize;
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            if node.value == *elem {
                num += 1;
            }

            current = node.next.as_ref();
        }
        num
         */

        self.iter().filter(|value| **value == *elem).count()
    }

    /// returen reference to the first kept item
    /// 
    /// # Example
    /// ```
    /// use basic::List;
    /// 
    /// let mut list = List::<i32>::new();
    /// list.push_front(1);
    /// assert_eq!(*list.head().unwrap(), 1);
    /// ```
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    /// returen mut reference to the first kept item
    /// 
    /// # Example
    /// ```
    /// use basic::List;
    /// 
    /// let mut list = List::<i32>::new();
    /// list.push_front(1);
    /// *list.head_mut().unwrap() = 2;
    /// assert_eq!(*list.head().unwrap(), 2);
    /// ```
    pub fn head_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }

    /// return reference to the last kept item
    /// 
    /// # Example
    /// ```
    /// use basic::List;
    /// 
    /// let mut list = List::<i32>::new();
    /// list.push_front(1);
    /// list.push_front(2);
    /// assert_eq!(*list.tail().unwrap(), 1);
    /// ```
    pub fn tail(&self) -> Option<&T> {
        /*
        if self.tail.is_null() {
            None
        }
        else {
            unsafe { Some(&(*self.tail).value) }
        } */

        unsafe { self.tail.as_ref().map(|node| &node.value) }
    }

    /// return mut reference to the last kept item
    /// 
    /// # Example
    /// ```
    /// use basic::List;
    /// 
    /// let mut list = List::<i32>::new();
    /// list.push_front(1);
    /// list.push_front(2);
    /// *list.tail_mut().unwrap() = 3;
    /// assert_eq!(*list.tail().unwrap(), 3);
    /// ```
    pub fn tail_mut(&mut self) -> Option<&mut T> {
        /*
        if self.tail.is_null() {
            None
        }
        else {
            unsafe { Some(&mut (*self.tail).value) }
        } */
        unsafe { self.tail.as_mut().map(|node| &mut node.value) }
    }

    /// check if the item is kept
    /// 
    /// # Example
    /// ```
    /// use basic::List;
    /// 
    /// let mut list = List::<i32>::new();
    /// list.push_front(1);
    /// list.push_front(2);
    /// list.push_front(3);
    /// assert!(list.contains(&2))
    /// ```
    pub fn contains(&self, item:&T) -> bool {

        /*
        let mut current = self.head.as_ref();

        while let Some(node) = current {

            if node.value == *item {
                return true;
            }
            current = node.next.as_ref();
        }
        false */

        self.iter().any(|value| *item == *value)
    }

    /// add item after element 
    /// 
    /// # Example
    /// ```
    /// use basic::List;
    /// 
    /// let mut list = List::<i32>::new();
    /// list.push_front(1);
    /// list.push_front(3);
    /// list.push_front(4);
    /// list.add_after(2, &3);
    /// 
    /// assert_eq!(4, list.rm_front().unwrap());
    /// assert_eq!(3, list.rm_front().unwrap());
    /// assert_eq!(2, list.rm_front().unwrap());
    /// assert_eq!(1, list.rm_front().unwrap());
    /// ```
    pub fn add_after(&mut self, item:T, elem:&T)  {
        let mut current = self.head.as_mut();

        while let Some(node) = current {
            if node.value == *elem {
                let is_last = node.next.is_none();
                node.next = Some(Box::new(Node{value:item, next:node.next.take()}));
                self.size += 1;
                if is_last {
                    self.tail = match node.next.as_mut() {
                        Some(node) => &mut ** node as *mut Node<T>,
                        None => std::ptr::null_mut(),
                    }
                }
                break;
            }

            current = node.next.as_mut();
        }
    }
    
    /// add item at the tail of the list
    /// 
    /// # Example
    /// ```
    /// use basic::List;
    /// 
    /// let mut list = List::<i32>::new();
    /// list.push_front(1);
    /// list.push_back(2);
    /// 
    /// assert_eq!(*list.tail().unwrap(), 2);
    /// assert_eq!(*list.head().unwrap(), 1);
    /// ```
    pub fn push_back(&mut self, item:T) {

        /* O(N)
        let mut current = &mut self.head;

        while let Some(node) = current {
            if node.next.is_none() {
               node.next = Some(Box::new(Node{value:item, next:None}));
                self.tail = match node.next.as_mut() {
                    Some(node) => &mut ** node as *mut Node<T>,
                    None => std::ptr::null_mut(),
                };
                return;
            }

            current = &mut node.next;
        }
       // it head is None
       self.push_front(item); */

        if self.tail.is_null() {
            self.push_front(item);
        }
        else {
            unsafe {
                let tail_node = &mut *self.tail;
                tail_node.next = Some(Box::new(Node::new(item)));
                self.tail = &mut **tail_node.next.as_mut().unwrap();
                self.size += 1;
            }
        }
       
    }
    
    /// return number of kempt items ?
    /// 
    /// # Example
    /// ```
    /// use basic::List;
    /// 
    /// let mut list = List::<i32>::new();
    /// assert_eq!(list.size(), 0);
    /// list.push_front(1);
    /// assert_eq!(list.size(), 1);
    /// let _ = list.rm_front();
    /// assert_eq!(list.size(), 0);
    /// ```
    pub fn size(&self) -> usize {
        self.size.clone()
     }

    /// function print list in format {...}
    /// 
    pub fn println(&self) {
        let mut current = &self.head;
        if current.is_none() {
            println!("[]");
        }
        else {
        print!("[");
            while let Some(node) = current {
                if node.next.is_none() {
                    println!("{:?}]", node.value);
                    return;
                }

                print!("{:?}, ", node.value);
                current = &node.next;
            }
        }
    }

}

impl<'a, T: std::fmt::Debug + std::cmp::PartialEq> Iterator for ListIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {

        if let Some(node) = self.next_node {
            self.next_node = node.next.as_deref();
            Some(&node.value)
        }
        else {
            None
        }
    }
}