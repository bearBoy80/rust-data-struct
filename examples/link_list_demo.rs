use data_struct::LinkedList;

fn main() {
    let mut linked_list: LinkedList<i32> = LinkedList::new();
    linked_list.push_tail(23);
    linked_list.push_tail(24);
    linked_list.push_tail(25);
    linked_list.push_tail(26);
    linked_list.push_back(27);
    println!("{:?}", linked_list);
}
