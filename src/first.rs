pub struct List {
    head: Link,
}

// Either empty (null) or a ptr to next Node
enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}
