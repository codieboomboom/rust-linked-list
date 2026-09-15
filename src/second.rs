// Notes:
// Option comes with some default method that is quite useful:
    // - Instead of replace(stuff, None), just use stuff.take()
    // - Instead of {None => None, Some(x) => Some(y)} we can use functional programming map() and closure. We don't need to ret Some(y), just map x -> y directly
    // If it is pointy, it is generic
    // There are 3 types of iterator: IntoIter => consumed/moved elements, cannot use after, "yield" T
    //                                  IterMut => borrow mutably, can used after, "yield" &mut T
    //                                  Iter => borrow shared, can used after, "yield" &T

    // Impl an iterator means defining struct to keep track of collection state being iterated
use std::mem;

pub struct List<T> {
    head: Link<T>,
}

// Either empty (null) or a ptr to next Node
type Link<T> = Option<Box<Node<T>>>; // type alias Link to Option of Box<Node>

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>); // wrap around List and consume it
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            elem: value,
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
        self.head.as_ref().map(|node| 
            {
                &node.elem
            }
        )
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| 
            {
                &mut node.elem
            }
        )
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref().map(|node| &*node)
        }
    }

// if apply liftime elision
/*
impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }
}
impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }
}

 */
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        //access fields of a tuple struct numerically
        self.0.pop() // pop the list (list is field 0)
    }
}


impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref().map(|node| &*node);
            &node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr_link = self.head.take();
        while let Some(mut boxed_node) = curr_link {
            curr_link = mem::take(&mut boxed_node.next);
            // matched boxed_node is no longer used here, so it will be out of scope and get dropped
            // but it's 'next' field have been set to Empty using take above
            // therefore no unbound recursion would happen here
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basic() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);

        assert_eq!(list.pop(), Some(2));

        list.push(3);
        list.push(4);

        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| {
            *value = 33
        });
        assert_eq!(list.peek(), Some(&33));
        assert_eq!(list.pop(), Some(33));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

}