// use std::ops::{Index, IndexMut};
use std::rc::Rc;
// use std::cell::Ref;
use std::cell::RefCell;
use std::fmt;
use std::ops::{Add, BitAnd};
use std::cmp::PartialEq;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[allow(dead_code)]
#[derive(Clone)]
pub struct DLinkedList<T>{
    size: usize,
    head: Link<T>,
    tail: Link<T>,
    // current_node: Option<T>, //for Index Trait
    //next_node: Link<T>,   //for iteration optimization Trait
    //next_count: usize, //for iteration optimization Trait
    //sorted: i8    //for sort optimization
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    next: Link<T>,
    prev: Link<T>,
}


#[allow(dead_code, unused_variables)]
impl <T> DLinkedList<T> {
    
    pub fn new() -> Self {
        DLinkedList {
            size: 0,
            head: None,
            tail: None,
        }
    }

    pub fn append(&mut self, other: &mut Self) {
        if self.len() == 0 {
            if other.len() > 0 {
                let moved_other = other;
                self.head = moved_other.head.clone();
                self.tail = moved_other.tail.clone();
                self.size = moved_other.len();
                moved_other.clear();
            }
        } else {
            if other.len() > 0 {
                let mut moved_other = other;

                let tail_self = self.tail.as_mut().unwrap();
                tail_self.borrow_mut().next = moved_other.head.clone();

                let tail_moved_other = moved_other.head.as_mut().unwrap();
                tail_moved_other.borrow_mut().prev = self.tail.clone();

                self.tail = moved_other.tail.clone();
                moved_other.head = None;

                self.size = self.size + moved_other.len();
                moved_other.clear();
            }
        }
        
    }

    pub fn is_empty(&self) -> bool {
        if self.size == 0 { true } else { false }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push_back(&mut self, data: T) {
        if self.size == 0 {
            let new_node = Rc::new(RefCell::new(Node {
                data: data,
                next: None,
                prev: None
            }));

            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(Rc::clone(&new_node));
            self.size += 1;

        } else {
            if self.size < (usize::max_value() - 1){
                let tail_node: Rc<RefCell<Node<T>>> = self.tail.take().unwrap();
                let new_node: Rc<RefCell<Node<T>>> = Rc::new(RefCell::new(Node {
                    data: data,
                    next: None,
                    prev: Some(Rc::clone(&tail_node)),
                }));

                self.tail = Some(Rc::clone(&new_node));

                let tail_node = &*tail_node;
                tail_node.borrow_mut().next = Some(Rc::clone(&new_node));
                
                self.size += 1;

            } else {
                panic!("Limit size exceeded!")
            }
        }
    }

    pub fn push_front(&mut self, data: T) {
        if self.size == 0 {
            let new_node = Rc::new(RefCell::new(Node {
                data: data,
                next: None,
                prev: None
            }));

            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(Rc::clone(&new_node));
            self.size += 1;

        } else {
            if self.size < (usize::max_value() - 1){
                let head_node: Rc<RefCell<Node<T>>> = self.head.take().unwrap();
                let new_node: Rc<RefCell<Node<T>>> = Rc::new(RefCell::new(Node {
                    data: data,
                    next: Some(Rc::clone(&head_node)),
                    prev: None,
                }));

                self.head = Some(Rc::clone(&new_node));

                let head_node = &*head_node;
                head_node.borrow_mut().prev = Some(Rc::clone(&new_node));
                
                self.size += 1;

            } else {
                panic!("Limit size exceeded!")
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.size > 0 {
            let node_pop = self.tail.take().unwrap();
            let node_p = &*node_pop;

            self.tail = Some(Rc::clone(node_p.borrow_mut().prev.as_ref().unwrap()));

            node_pop.borrow_mut().prev = None;
            
            let prev_node = self.tail.as_ref().unwrap();
            let prev_node = &*prev_node;
            prev_node.borrow_mut().next = None;
            
            let node_pop = match Rc::try_unwrap(node_pop) {
                Ok(number)  => number,
                Err(e) => return None,
            };
            let node_pop = node_pop.into_inner();

            self.size -= 1;

            Some(node_pop.data)

        } else {
            None
        }
    }    

    pub fn clear(&mut self) {
        self.size = 0;
        if self.head.is_some() {
            self.head = None;
            self.tail = None;
        }
    }
}

impl <T> DLinkedList<T> where T: Clone {

    pub fn new_with_data(capacity: usize, data: T) -> Self {
        let mut ary = DLinkedList::new();
        if capacity > 0 && capacity < (usize::max_value() - 1){
            for i in 0..capacity {
                if i == 0 {
                    let new_node = Rc::new(RefCell::new(Node {
                        data: T::clone(&data),
                        next: None,
                        prev: None
                    }));

                    ary.head = Some(Rc::clone(&new_node));
                    ary.tail = Some(Rc::clone(&new_node));
                    ary.size += 1;

                } else {
                    let node: Rc<RefCell<Node<T>>> = ary.tail.take().unwrap();
                    let new_node: Rc<RefCell<Node<T>>> = Rc::new(RefCell::new(Node {
                        data: T::clone(&data),
                        next: None,
                        prev: Some(Rc::clone(&node)),
                    }));

                    ary.tail = Some(Rc::clone(&new_node));

                    let node = &*node;
                    node.borrow_mut().next = Some(Rc::clone(&new_node));
                    
                    ary.size += 1;
                }
            }
        } else {
            panic!("Limit size exceeded!")
        }
        ary
    }

    pub fn at(&self, index: usize) -> T {
        if (index + 1) > self.size || self.size == 0 {
            assert_eq!(self.size, 0, "");
            panic!("Error: index out range")
        } else {
            if index == 0 {
                let node = self.head.as_ref().unwrap();
                let node = node.as_ref();
                let node = node.borrow();
                let data = (&node.data).clone();
                data
            } else {
                let mut node_iter: Rc<RefCell<Node<T>>> = Rc::clone(self.head.as_ref().unwrap());
                for _i in 1..=index {
                    let node = node_iter.as_ref();
                    let node = node.borrow().clone();
                    if node.next.is_some() {
                        node_iter = Rc::clone(node.next.as_ref().unwrap());
                    }
                }
                let node = node_iter.as_ref();
                let node = node.borrow();
                let data = (&node.data).clone();
                data
            }
        }
    }

    pub fn resize_with_value(&mut self, new_len: usize, value: T){
        if new_len < self.len() {
            for _i in (new_len - 1)..self.len() {
                self.pop_back();
            }
        } else if self.len() > new_len {
            for _i in (self.len() - 1)..(self.len() + new_len) {
                self.push_back(value.clone());
            }
        }
    }

    pub fn concat(&self, other: &Self) -> Self {
        let mut dlist: DLinkedList<T> = DLinkedList::<T>::new();
        let mut node_iter: Rc<RefCell<Node<T>>> = Rc::clone(self.head.as_ref().unwrap());
        for _i in 0..self.len() {
            let node = node_iter.as_ref();
            let node = node.borrow().clone();
            dlist.push_back(node.data.clone());
            if node.next.is_some() {
                node_iter = Rc::clone(node.next.as_ref().unwrap());
            }
        }
        for i in 0..other.len() {
            dlist.push_back(other.at(i));
        }
        dlist
    }
}

impl <T> DLinkedList<T> where T: Clone + PartialEq {

    pub fn has(&self, value: &T) -> bool {
        if self.size == 0 { 
            false 
        } else { 
            let mut flag = false;
            let mut node_iter: Rc<RefCell<Node<T>>> = Rc::clone(self.head.as_ref().unwrap());
            for _i in 0..self.len() {
                let node = node_iter.as_ref();
                let node = node.borrow().clone();
                if &node.data == value {
                    flag = true;
                    break;
                }
                if node.next.is_some() {
                    node_iter = Rc::clone(node.next.as_ref().unwrap());
                }
            }
            flag
        }
    }
}

impl <T> DLinkedList<T> where T: Default{

    pub fn new_with_capacity(capacity: usize) -> Self {
        let mut ary = DLinkedList::new();
        if capacity > 0 && capacity < (usize::max_value() - 1){
            for i in 0..capacity {
                if i == 0 {
                    let new_node = Rc::new(RefCell::new(Node {
                        data: T::default(),
                        next: None,
                        prev: None,
                    }));

                    ary.head = Some(Rc::clone(&new_node));
                    ary.tail = Some(Rc::clone(&new_node));
                    ary.size += 1;

                } else {
                    let node: Rc<RefCell<Node<T>>> = ary.tail.take().unwrap();
                    let new_node: Rc<RefCell<Node<T>>> = Rc::new(RefCell::new(Node {
                        data: T::default(),
                        next: None,
                        prev: Some(Rc::clone(&node)),
                    }));

                    ary.tail = Some(Rc::clone(&new_node));

                    let node = &*node;
                    node.borrow_mut().next = Some(Rc::clone(&new_node));
                    
                    ary.size += 1;
                }
            }
        } else {
            panic!("Limit size exceeded!")
        }
        ary
    }

    pub fn resize_default(&mut self, new_len: usize) {
        if new_len < self.len() {
            for _i in (new_len - 1)..self.len() {
                self.pop_back();
            }
        } else if self.len() > new_len {
            for _i in (self.len() - 1)..(self.len() + new_len) {
                self.push_back(T::default());
            }
        }
    }

    pub fn insert(&mut self, pos: usize, data: T) where T: Clone {
        if self.size == 0 { //array's size is 0
            let new_node = Rc::new(RefCell::new(Node {
                data: data,
                next: None,
                prev: None
            }));

            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(Rc::clone(&new_node));
            self.size += 1;
        } else {
            if self.size < (usize::max_value() - 1) && pos < (usize::max_value() - 1) {
                if pos <= (self.size - 1){
                    let mut node_iter: Rc<RefCell<Node<T>>> = Rc::clone(self.head.as_ref().unwrap());
                    for _i in 1..pos {
                        let node = node_iter.as_ref();
                        let mut node = node.borrow_mut().clone();
                        node_iter = Rc::clone(node.next.as_mut().unwrap());
                    }

                    let node = node_iter.as_ref();
                    let mut node = node.borrow_mut();
                    let node_next: Rc<RefCell<Node<T>>> = Rc::clone(node.next.as_ref().unwrap());
                    let node_next = node_next.as_ref();
                    let mut node_next = node_next.borrow_mut();

                    let new_node = Rc::new(RefCell::new(Node {
                        data: data,
                        next: node.next.clone(),
                        prev: Some(Rc::clone(&node_iter)),
                    }));

                    node.next = Some(Rc::clone(&new_node));
                    node_next.prev = Some(Rc::clone(&new_node));

                } else {

                    let begin = self.size;
                    let end = pos;
                    for _i in begin..end {
                        self.push_back(T::default());
                    }
                    self.push_back(data);
                }

                self.size += 1;

            } else {
                panic!("Limit size exceeded!")
            }
        }
    }
}

// impl <T> Index<usize> for DLinkedList<T> where T: Clone + AsRef<T>{
//     type Output = T;

//     fn index(&self, index: usize) -> &T {
//         self.head.as_ref().unwrap()
//     }
// }

// impl <T> IndexMut<usize> for DLinkedList<T> where T: Clone + AsRef<T>{
//     fn index_mut(&mut self, index: usize) -> &mut T {
//         self.current_node = Some(self.at(index));
//         self.current_node.as_mut().unwrap()
//     }
// }

// Notice that the implementation uses the associated type `Output`.
impl<T: Add<Output = T>> Add for DLinkedList<T> where T: Clone{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.concat(&other)
    }
}

impl <T> BitAnd for DLinkedList<T> where T: Clone + PartialEq{
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        let mut dlist: DLinkedList<T> = DLinkedList::<T>::new();
        if self.len() != 0 || other.len() != 0 {
            for i in 0..self.len() {
                for j in 0..other.len() {
                    if self.at(i) == other.at(j) {
                        if !dlist.has(&self.at(i)) {
                            dlist.push_back(self.at(i));
                        }
                    }
                }
            }
        }
        dlist
    }
}

impl <T> Default for DLinkedList<T> {
    fn default() -> Self {
        DLinkedList {
            size: 0,
            head: None,
            tail: None,
        }
    }
}

impl <'a, T> fmt::Display for DLinkedList<T> where T: Clone + fmt::Display {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.head {
            None => {
                writeln!(f, "[]")
            },
            _ => {
                let head = self.head.as_ref();
                write!(f, "[");
                match head {
                    None => {},
                    _ => {
                        let head = head.as_ref().unwrap();
                        let node_iter = &**head;
                        let node_iter = &node_iter.borrow();
                        let mut data = (*node_iter).clone();
                        for i in 0..self.size {
                            let value = data.data.clone();
                            write!(f, "{}", value);

                            if i < self.size - 1 {
                                write!(f, ", ");
                            }
                            
                            let next = data.clone();
                            let next = next.next.as_ref();
                            match next {
                                None => {},
                                _ => {
                                    let next = next.as_ref().unwrap();
                                    let next: &Rc<RefCell<Node<T>>> = &*next;
                                    let next = &**next;
                                    let next = &next.borrow();
                                    data = (*next).clone(); 
                                }
                            };
                        }
                    }
                }
                
                writeln!(f, "]")
            },
        } 
    }
}

impl<T> AsRef<DLinkedList<T>> for DLinkedList<T> {
    fn as_ref(&self) -> &DLinkedList<T> {
        self
    }
}

impl<T> AsMut<DLinkedList<T>> for DLinkedList<T> {
    fn as_mut(&mut self) -> &mut DLinkedList<T> {
        self
    }
}

impl<T> From<Vec<T>> for DLinkedList<T> where T: Clone {
    fn from(vec: Vec<T>) -> DLinkedList<T> {
        let mut dlist = DLinkedList::<T>::new();
        for i in 0..vec.len() {
            dlist.push_back(vec[i].clone());
        }
        dlist
    }
}

#[allow(dead_code, unused_variables)]
#[cfg(test)]
mod tests {
    use super::DLinkedList;
    
    #[test]
    fn init() {
        let mut dlist = DLinkedList::<i32>::new();
        println!("dlist = {}", dlist);
        dlist.push_back(2);
        println!("{}", dlist);
        dlist.push_back(3);
        println!("{}", dlist);

        let dlist2 = DLinkedList::<i32>::new_with_capacity(10);
        println!("dlist = {}", dlist2);

        let dlist3 = DLinkedList::<i32>::new_with_data(10, 100);
        println!("dlist = {}", dlist3);
    }

    #[test]
    fn push_pop_front_any() {
        let mut array = DLinkedList::<i32>::new();
        println!("{}", array);

        // Check empty Array behaves right
        assert_eq!(array.pop_back(), None);

        // Populate Array
        array.push_front(1);
        array.push_back(2);
        array.push_back(3);
        array.push_front(4);
        array.push_front(5);
        array.push_back(6);
        array.push_front(7);

        assert_eq!(array.pop_back(), Some(6));
        // println!("{:?}", array);

        // assert_eq!(array.pop_back(), Some(3));

        // Push some more just to make sure nothing's corrupted
        // array.push_back(4);
        // array.push_back(5);

        println!("{}", array);
    }

    #[test]
    fn indexing() {
        let mut dlist = DLinkedList::<i32>::new();
        dlist.push_front(1);
        dlist.push_back(2);
        dlist.push_back(3);
        dlist.push_front(4);
        dlist.push_front(5);
        dlist.push_back(6);
        dlist.push_front(7);

        assert_eq!(dlist.at(0), 7);
        assert_eq!(dlist.at(1), 5);
        assert_eq!(dlist.at(2), 4);
        assert_eq!(dlist.at(3), 1);
        assert_eq!(dlist.at(4), 2);
        assert_eq!(dlist.at(5), 3);
        assert_eq!(dlist.at(6), 6);

    }

    #[test]
    fn appending() {

        let mut dlist1 = DLinkedList::<i32>::new();
        dlist1.push_back(1);
        dlist1.push_back(2);
        dlist1.push_back(3);

        let mut dlist2 = DLinkedList::<i32>::new();
        dlist2.push_back(4);
        dlist2.push_back(5);
        dlist2.push_back(6);

        assert_eq!(dlist1.len(), 3);
        assert_eq!(dlist2.len(), 3);

        dlist1.append(&mut dlist2);

        assert_eq!(dlist1.len(), 6);
        assert_eq!(dlist2.len(), 0);

        assert_eq!(dlist1.at(0), 1);
        assert_eq!(dlist1.at(1), 2);
        assert_eq!(dlist1.at(2), 3);
        assert_eq!(dlist1.at(3), 4);
        assert_eq!(dlist1.at(4), 5);
        assert_eq!(dlist1.at(5), 6);


        assert_eq!(dlist1.has(&4), true);

    }

    #[test]
    fn intersection() {

        let mut dlist1 = DLinkedList::<i32>::new();
        dlist1.push_back(1);
        dlist1.push_back(2);
        dlist1.push_back(2);
        dlist1.push_back(8);

        let mut dlist2 = DLinkedList::<i32>::new();
        dlist2.push_back(3);
        dlist2.push_back(1);
        dlist2.push_back(2);

        let dlist3 = dlist1 & dlist2;

        assert_eq!(dlist3.len(), 2);
        assert_eq!(dlist3.at(0), 1);
        assert_eq!(dlist3.at(1), 2);

    }
}
