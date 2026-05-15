use rust_leet::leet::linked_list_level_2::LinkedList;
fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);
    list.push(6);

    println!("{:#?}", list);
    
    for val in list.iter_mut() {
        *val = 3
    }

    println!("{:#?}", list);
}