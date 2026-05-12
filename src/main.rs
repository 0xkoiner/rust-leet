use rust_leet::leet::linked_list_level_2::LinkedList;                                                                  
                                                                                                                        
fn main() {                                                                                                            
    let mut list = LinkedList::new();                 
    list.push(1);
    list.push(1);
    list.push(1);
    list.push(1);
    println!("{:#?}", list);   // ← will stack overflow at runtime                                                      
}