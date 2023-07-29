/// simple heap D-way
///
pub type Data<T> = Vec<Node<T>>;
pub type Index = usize;
pub type Priority = u64;

// static const D: Index = 2;
const TOP: Index = 0;
const FIRST_CHILD_INDEX: Index = 1;

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub value: T,
    pub priority: Priority,
}

impl<T> PartialEq for Node<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Node<T>) -> bool {
        self.value == other.value && self.priority == other.priority
    }
}

#[derive(Debug)]
pub struct Heap<T, const D: Index = 2> {
    data: Data<T>,
}

impl<T, const D: Index> Heap<T, D>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    /// create new heap
    pub fn new() -> Self {
        Self {
            data: Vec::<Node<T>>::new(),
        }
    }

    /// create new heap from vector
    pub fn new_from(mut data: Data<T>) -> Self {
        heapify::<T, D>(&mut data);
        Self { data }
    }

    /// create new heap with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        let data = Vec::<Node<T>>::with_capacity(capacity);
        Self { data }
    }

    /// init from vector
    pub fn from(mut self, data: Data<T>) -> Self {
        self.data = data;
        heapify::<T, D>(&mut self.data);
        self
    }

    /// set new heap from vector
    pub fn new_heap(&mut self, data: Data<T>) {
        self.data = data;
        heapify::<T, D>(&mut self.data);
    }

    /// check is heap empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// number of parameters
    pub fn len(&self) -> usize {
        self.data.len()
    }

    // consider return Option<T>
    pub fn peek(&self) -> T {
        if self.data.is_empty() {
            panic!("empty heap")
        }
        self.data[TOP].value.clone()
    }

    // consider return Option<T>
    pub fn top(&mut self) -> T {
        let v = self.peek();
        self.data.swap_remove(TOP);
        if !self.data.is_empty() {
            push_down::<T, D>(&mut self.data, TOP);
        }

        v
    }

    pub fn insert(&mut self, value: T, priority: Priority) {
        self.data.push(Node { value, priority });
        bubble_up::<T, D>(&mut self.data);
    }

    pub fn remove(&mut self, value: T) {
        match self.data.len() {
            0 => return, // panic!?
            1 => self.data.clear(),
            _ => {
                if let Some(index) = self.find(&value) {
                    self.data.swap_remove(index);
                    push_down::<T, D>(&mut self.data, index);
                }
            } // consider panic! otherwise
        }
    }

    pub fn update(&mut self, value: T, priority: Priority) {
        if let Some(index) = self.find(&value) {
            let old_priority = self.data[index].priority;
            self.data[index].priority = priority;

            if old_priority > priority {
                bubble_up_index::<T, D>(&mut self.data, index);
            } else {
                push_down::<T, D>(&mut self.data, index);
            }
        }
    }

    fn find(&self, value: &T) -> Option<Index> {
        self.data.iter().position(|node| node.value == *value)
    }
} // Heap

fn parent_index<const D: Index>(index: Index) -> Index {
    assert!(index > 0);
    (index - 1) / D
}

fn child_index<const D: Index>(index: Index, num: Index) -> Index {
    assert!(num > 0 && num <= D);
    D * index + num
}

fn first_child_index<const D: Index>(index: Index) -> Index {
    child_index::<D>(index, FIRST_CHILD_INDEX)
}

fn bubble_up_index<T, const D: Index>(data: &mut [Node<T>], index: Index)
where
    T: Clone + std::fmt::Debug,
{
    assert!(data.len() > index);

    let current_item = data[index].clone();
    let mut index = index;

    let cmp = |a, b| a > b;

    while index > 0 {
        let parent_id = parent_index::<D>(index);
        if cmp(data[parent_id].priority, current_item.priority) {
            data[index] = data[parent_id].clone();
            index = parent_id;
        } else {
            break;
        }
    }

    data[index] = current_item;
}

fn bubble_up<T, const D: Index>(data: &mut [Node<T>])
where
    T: Clone + std::fmt::Debug,
{
    bubble_up_index::<T, D>(data, data.len() - 1);
}

fn highest_priority_child<T, const D: Index>(
    data: &[Node<T>],
    index: Index,
) -> Option<(Index, Node<T>)>
where
    T: Clone + std::fmt::Debug,
{
    use std::cmp::min;
    let mut child_id = first_child_index::<D>(index);

    let cmp = |a, b| a > b; // TODO: pass as param

    if child_id < data.len() {
        let child_index_max = min(child_id + D, data.len());
        let mut child_prio = data[child_id].priority;

        for id in child_id + 1..child_index_max {
            if !cmp(data[id].priority, child_prio) {
                child_id = id;
                child_prio = data[id].priority;
            }
        }

        Some((child_id, data[child_id].clone()))
    } else {
        None
    }
}

fn push_down<T, const D: Index>(data: &mut [Node<T>], index: Index)
where
    T: Clone + std::fmt::Debug,
{
    let mut index = index;
    let current_item = data[index].clone();

    let cmp = |a, b| a > b; // TODO pass a param

    while let Some((child_id, child_item)) = highest_priority_child::<T, D>(data, index) {
        if cmp(current_item.priority, child_item.priority) {
            data[index] = child_item;
            index = child_id;
        } else {
            break;
        }
    }

    data[index] = current_item;
}

pub fn heapify<T, const D: Index>(data: &mut [Node<T>])
where
    T: Clone + std::fmt::Debug,
{
    for i in (0..data.len() / D + 1).rev() {
        push_down::<T, D>(data, i);
    }
}

#[cfg(test)]
mod prv_test {

    #[test]
    fn parent_index() {
        use super::*;

        {
            const D: Index = 2;
            assert_eq!(parent_index::<D>(1), 0);
            assert_eq!(parent_index::<D>(2), 0);
            assert_eq!(parent_index::<D>(3), 1);
            assert_eq!(parent_index::<D>(4), 1);
            assert_eq!(parent_index::<D>(5), 2);
            assert_eq!(parent_index::<D>(6), 2);
            assert_eq!(parent_index::<D>(7), 3);
            assert_eq!(parent_index::<D>(8), 3);
            assert_eq!(parent_index::<D>(9), 4);
        }

        {
            const D: Index = 3;
            assert_eq!(parent_index::<D>(1), 0);
            assert_eq!(parent_index::<D>(2), 0);
            assert_eq!(parent_index::<D>(3), 0);
            assert_eq!(parent_index::<D>(4), 1);
            assert_eq!(parent_index::<D>(5), 1);
            assert_eq!(parent_index::<D>(6), 1);
            assert_eq!(parent_index::<D>(7), 2);
            assert_eq!(parent_index::<D>(8), 2);
            assert_eq!(parent_index::<D>(9), 2);
        }
    }

    #[test]
    fn child_index() {
        use super::*;

        {
            const D: Index = 2;
            assert_eq!(child_index::<D>(0, 1), 1);
            assert_eq!(child_index::<D>(0, 2), 2);
            assert_eq!(child_index::<D>(1, 1), 3);
            assert_eq!(child_index::<D>(1, 2), 4);
            assert_eq!(child_index::<D>(2, 1), 5);
            assert_eq!(child_index::<D>(2, 2), 6);
            assert_eq!(child_index::<D>(3, 1), 7);
            assert_eq!(child_index::<D>(3, 2), 8);
            assert_eq!(child_index::<D>(4, 1), 9);
        }

        {
            const D: Index = 3;
            assert_eq!(child_index::<D>(0, 1), 1);
            assert_eq!(child_index::<D>(0, 2), 2);
            assert_eq!(child_index::<D>(0, 3), 3);
            assert_eq!(child_index::<D>(1, 1), 4);
            assert_eq!(child_index::<D>(1, 2), 5);
            assert_eq!(child_index::<D>(1, 3), 6);
            assert_eq!(child_index::<D>(2, 1), 7);
            assert_eq!(child_index::<D>(2, 2), 8);
            assert_eq!(child_index::<D>(2, 3), 9);
        }
    }

    #[test]
    fn first_child_index() {
        use super::*;

        {
            const D: Index = 2;
            assert_eq!(first_child_index::<D>(0), 1);
            assert_eq!(first_child_index::<D>(1), 3);
            assert_eq!(first_child_index::<D>(2), 5);
            assert_eq!(first_child_index::<D>(3), 7);
            assert_eq!(first_child_index::<D>(4), 9);
            assert_eq!(first_child_index::<D>(5), 11);
            assert_eq!(first_child_index::<D>(6), 13);
            assert_eq!(first_child_index::<D>(7), 15);
            assert_eq!(first_child_index::<D>(8), 17);
            assert_eq!(first_child_index::<D>(9), 19);
        }

        {
            const D: Index = 3;
            assert_eq!(first_child_index::<D>(0), 1);
            assert_eq!(first_child_index::<D>(1), 4);
            assert_eq!(first_child_index::<D>(2), 7);
            assert_eq!(first_child_index::<D>(3), 10);
            assert_eq!(first_child_index::<D>(4), 13);
            assert_eq!(first_child_index::<D>(5), 16);
            assert_eq!(first_child_index::<D>(6), 19);
            assert_eq!(first_child_index::<D>(7), 22);
            assert_eq!(first_child_index::<D>(8), 25);
            assert_eq!(first_child_index::<D>(9), 28);
        }
    }

    #[test]
    fn bubble_up_index() {
        use super::*;

        type Item = i32;
        const D: Index = 2;

        let mut data: Data<Item> = vec![
            Node {
                value: 1,
                priority: 1,
            },
            Node {
                value: 2,
                priority: 2,
            },
            Node {
                value: 3,
                priority: 3,
            },
            Node {
                value: 4,
                priority: 4,
            },
            Node {
                value: 5,
                priority: 5,
            },
            Node {
                value: 6,
                priority: 0,
            },
        ];

        let expected: Data<Item> = vec![
            Node {
                value: 6,
                priority: 0,
            },
            Node {
                value: 2,
                priority: 2,
            },
            Node {
                value: 1,
                priority: 1,
            },
            Node {
                value: 4,
                priority: 4,
            },
            Node {
                value: 5,
                priority: 5,
            },
            Node {
                value: 3,
                priority: 3,
            },
        ];

        let last = data.len() - 1;

        bubble_up_index::<Item, D>(&mut data, last);

        assert_eq!(data.len(), expected.len());

        for i in 0..data.len() {
            assert_eq!(data[i].value, expected[i].value);
            assert_eq!(data[i].priority, expected[i].priority);
        }

        // TODO add D3, Item = String  ...
    }

    #[test]
    fn highest_priority_child() {
        use super::*;

        type Item = i32;
        const D: Index = 2;

        let data: Data<Item> = vec![
            Node {
                value: 6,
                priority: 0,
            },
            Node {
                value: 2,
                priority: 2,
            },
            Node {
                value: 1,
                priority: 1,
            },
            Node {
                value: 4,
                priority: 4,
            },
            Node {
                value: 5,
                priority: 5,
            },
            Node {
                value: 3,
                priority: 3,
            },
        ];

        if let Some((index, node)) = highest_priority_child::<Item, D>(&data, 0) {
            assert_eq!(index, 2);
            assert_eq!(node.value, 1);
            assert_eq!(node.priority, 1);
            assert_eq!(data[index].value, 1);
            assert_eq!(data[index].priority, 1);
        } else {
            assert!(false);
        }

        if let Some((index, node)) = highest_priority_child::<Item, D>(&data, 1) {
            assert_eq!(index, 3);
            assert_eq!(node.value, 4);
            assert_eq!(node.priority, 4);
            assert_eq!(data[index].value, 4);
            assert_eq!(data[index].priority, 4);
        } else {
            assert!(false);
        }

        if let Some((_, _)) = highest_priority_child::<Item, D>(&data, 5) {
            assert!(false);
        }
    }

    #[test]
    fn push_down() {
        use super::*;

        {
            type Item = i32;
            const D: Index = 2;

            let mut data: Data<Item> = vec![
                Node {
                    value: 6,
                    priority: 0,
                },
                Node {
                    value: 2,
                    priority: 2,
                },
                Node {
                    value: 1,
                    priority: 1,
                },
                Node {
                    value: 4,
                    priority: 4,
                },
                Node {
                    value: 5,
                    priority: 5,
                },
                Node {
                    value: 3,
                    priority: 3,
                },
            ];

            push_down::<Item, D>(&mut data, 0);

            assert_eq!(data[0].value, 6);
            assert_eq!(data[0].priority, 0);

            data.swap_remove(0);

            push_down::<Item, D>(&mut data, 0);

            assert_eq!(data[0].value, 1);
            assert_eq!(data[0].priority, 1);
        }
        {
            type Item = String;
            const D: Index = 2;

            let mut data: Data<Item> = vec![
                Node {
                    value: String::from("6"),
                    priority: 0,
                },
                Node {
                    value: String::from("2"),
                    priority: 2,
                },
                Node {
                    value: String::from("1"),
                    priority: 1,
                },
                Node {
                    value: String::from("4"),
                    priority: 4,
                },
                Node {
                    value: String::from("5"),
                    priority: 5,
                },
                Node {
                    value: String::from("3"),
                    priority: 3,
                },
            ];

            push_down::<Item, D>(&mut data, 0);

            assert_eq!(data[0].value, String::from("6"));
            assert_eq!(data[0].priority, 0);

            data.swap_remove(0);

            push_down::<Item, D>(&mut data, 0);

            assert_eq!(data[0].value, String::from("1"));
            assert_eq!(data[0].priority, 1);
        }
    }
}
