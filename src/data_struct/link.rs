use std::{
    fmt::{self, Debug},
    ptr,
};

pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}
impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let next_addr = self.next.as_ref().map_or("None".to_string(), |next| {
            let addr = ptr::addr_of!(*next);
            format!("{:p}", addr)
        });
        f.debug_struct("Node")
            .field("data", &self.data)
            .field("next", &next_addr)
            .finish()
    }
}

pub struct LinkedList<T> {
    pub len: usize,
    pub head: Option<Box<Node<T>>>,
}
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { len: 0, head: None }
    }
    //添加node，在头部添加新的数据
    pub fn push_head(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.len += 1;
    }
    //在链表尾部添加数据
    pub fn push_tail(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: None });
        let mut tail = &mut self.head;

        loop {
            match tail {
                Some(node) => {
                    tail = &mut node.next;
                }
                None => {
                    *tail = Some(new_node);
                    break;
                }
            }
        }
        self.len += 1;
    }
    /* 利用ref关键字来实现链表尾部添加数据
     */
    pub fn push_back(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: None });

        if let Some(ref mut head) = self.head {
            let mut current = head;

            while let Some(ref mut next) = current.next {
                current = next;
            }

            current.next = Some(new_node);
        } else {
            self.head = Some(new_node);
        }

        self.len += 1;
    }

    pub fn pop_head(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            self.len -= 1;
            node.data
        })
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = self.head.as_deref();
        let mut list = vec![];

        while let Some(node) = current {
            list.push(format!("{:?}", node));
            current = node.next.as_deref();
        }

        write!(f, "{:?}", list)
    }
}
impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
