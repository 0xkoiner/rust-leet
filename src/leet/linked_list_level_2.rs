use std::ops::Drop;
use std::fmt::{Debug, Formatter, Result};

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>, // bytes8 -> size_of::<T>() + 8 + (padding)
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current_node: Option<Box<Node<T>>> = self.head.take();

        while let Some(mut node) = current_node {
            current_node = node.next.take();
        }
    }
    
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_struct("Node")
            .field("value", &self.value)
            .field("next", &self.next)
            .finish()
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_struct("List {")
            .field("head", &self.head.as_ref())
            .finish()
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self { Self::new() }
}

#[warn(clippy::new_without_default)]
impl<T> LinkedList<T>  {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node {
            value, next: self.head.take()
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take()  {
            Some(node) => { 
                self.head = node.next;
                Some(node.value)
            },
            None => None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref()
            .map(|node| &node.value)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

