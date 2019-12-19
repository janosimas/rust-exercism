use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Default)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut node = &self.head;
        while node.is_some() {
            count += 1;
            node = &node.as_ref().unwrap().next;
        }

        count
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node{
            data: _element,
            next: self.head.take()
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut head = self.head.take();

        if let Some(ref mut b) = head {
            self.head = b.next.take()
        }

        head.map(|b| b.data)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|b| &b.data)
    }

    pub fn rev(mut self) -> SimpleLinkedList<T>
    {
        let mut list = SimpleLinkedList::new();
        while let Some(e) = self.pop() {
            list.push(e);
        }

        list
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        _iter.into_iter().for_each(|elem| list.push(elem));
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T>
where
    T: Copy,
{
    fn into(mut self) -> Vec<T> {
        let mut vector = vec![];

        while let Some(e) = self.pop() {
            vector.push(e);
        }
        vector.reverse();
        vector
    }
}
