struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>, // bytes8 -> size_of::<T>() + 8 + (padding)
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
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

