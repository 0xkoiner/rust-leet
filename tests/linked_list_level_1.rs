use rust_leet::leet::linked_list_level_1::LinkedList;

enum ListState { Empty, NonEmpty }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_list() {
        let list: LinkedList<i32> = new_list(ListState::Empty);
        assert_eq!(list.peek(), None);
    }

    #[test]
    fn push() {
        let mut list: LinkedList<i32> = new_list(ListState::Empty);

        list.push(11);

        let peek: &i32 = list.peek().unwrap();

        assert_eq!(peek, &11);
    }

    #[test]
    fn pop() {
        let mut list: LinkedList<i32> = new_list(ListState::NonEmpty);

        list.push(11);
        list.push(12);
        list.push(13);

        let val = list.pop().unwrap();

        assert_eq!(val, 13);
    }

    #[test]
    fn peek() {
        let list: LinkedList<i32> = new_list(ListState::NonEmpty);
        let peek: &i32 = list.peek().unwrap();

        assert_eq!(peek, &10);
    }

    #[test]
    fn is_empty() {
        let mut list: LinkedList<i32> = new_list(ListState::Empty);
        assert_eq!(list.is_empty(), true);

        list.push(10);
        assert_eq!(list.is_empty(), false);
    }
}

fn new_list(is_empty: ListState) -> LinkedList<i32>{
    match is_empty {
        ListState::Empty     => LinkedList::new(),
        ListState::NonEmpty  => {
            let mut list = LinkedList::new();
            list.push(10);

            list
        },
    }
}