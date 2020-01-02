use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    // Delete this field
    // dummy is needed to avoid unused parameter error during compilation
    head: Option<Box<Node<T>>>,
}

pub struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut node = &self.head;
        let mut count = 0;
        loop {
            match node {
                Some(boxed_node) => {
                    count += 1;
                    node = &boxed_node.next
                }
                None => break,
            }
        }
        count
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            elem: element,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut v: Vec<T> = self.into();
        v.reverse();
        SimpleLinkedList::from_iter(v.into_iter())
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for elem in iter {
            list.push(elem);
        }
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

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut v: Vec<T> = Vec::new();
        let mut node = self.head;
        loop {
            match node {
                Some(mut boxed_node) => {
                    v.push(boxed_node.elem);
                    node = boxed_node.next.take()
                }
                None => break,
            }
        }
        v.reverse();
        v
    }
}
