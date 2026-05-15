use std::cell::RefCell;
use std::rc::{Rc, Weak};

type NextNode<T> = Option<Rc<RefCell<Node<T>>>>;
type PrevNode<T> = Option<Weak<RefCell<Node<T>>>>;

struct Node<T> {
    value: T,
    next: NextNode<T>,
    prev: PrevNode<T>,
}

pub struct DLinkedList<T> {
    head: NextNode<T>,
    tail: NextNode<T>,
    len: usize,
}

impl<T> DLinkedList<T> {
    pub fn new() -> Self {
        Self { 
            head: None, 
            tail: None, 
            len: 0 
        }
    }

    pub fn push_front(&mut self, value: T) {                                                                                         
        let new_node: Rc<RefCell<Node<T>>> = Rc::new(RefCell::new(Node {                                                                                 
            value,                                                                                                                   
            next: self.head.take(),                                                     
            prev: None,                                                                                                              
        }));

        match new_node.borrow().next.as_ref() {                                                                                    
            Some(old_head) => {                                                                                                                                                                        
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));                                                         
            }
            None => {                                                                                                                                                                   
                self.tail = Some(Rc::clone(&new_node));                                                                              
            }
        }                                                                                                                            

        self.head = Some(new_node);                                                                                                  
        self.len += 1;
    }

    pub fn push_end(&mut self, value: T) {
        let new_node: Rc<RefCell<Node<T>>> = Rc::new(RefCell::new(Node {                                                                                   
            value,                                                                                                                   
            next: None,                                                                                                              
            prev: self.tail.as_ref().map(Rc::downgrade),                                                                           
        }));   

        match self.tail.take() {                                                                                                     
            Some(old_tail) => {                                                    
                old_tail.borrow_mut().next = Some(Rc::clone(&new_node));                                                             
            }                                                                                                                        
            None => {                                                                                                                
                self.head = Some(Rc::clone(&new_node));                                                                              
            }                                                                                                                        
        }                                                                                                                                                                                 
        self.tail = Some(new_node);                                                                                              
        self.len += 1;
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none() && self.tail.is_none()
    } 
}