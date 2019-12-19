mod lib;
use lib::SimpleLinkedList;

fn main() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    assert_eq!(list.peek(), None, "No element should be contained in list");
    list.push(2);
    assert_eq!(list.peek(), Some(&2), "Element must be 2");
    assert_eq!(list.peek(), Some(&2), "Element must be still 2");
    list.push(3);
    assert_eq!(list.peek(), Some(&3), "Head element is now 3");
    assert_eq!(list.pop(), Some(3), "Element must be 3");
    assert_eq!(list.peek(), Some(&2), "Head element is now 2");
    assert_eq!(list.pop(), Some(2), "Element must be 2");
    assert_eq!(list.peek(), None, "No element should be contained in list");
}