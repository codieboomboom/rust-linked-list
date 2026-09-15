// Notes:
// Option comes with some default method that is quite useful:
    // - Instead of replace(stuff, None), just use stuff.take()
    // - Instead of {None => None, Some(x) => Some(y)} we can use functional programming map() and closure. We don't need to ret Some(y), just map x -> y directly
use std::mem;

pub struct List {
    head: Link,
}

// Either empty (null) or a ptr to next Node
type Link = Option<Box<Node>>; // type alias Link to Option of Box<Node>

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            elem: value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
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
}