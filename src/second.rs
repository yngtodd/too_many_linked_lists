type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
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
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(_boxed_node) = cur_link {
            cur_link = self.head.take();
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list is OK
        assert_eq!(list.pop(), None);
        assert_eq!(list.peek(), None);

        // Populate list
        list.push(0.);
        list.push(1.);
        list.push(2.);

        assert_eq!(list.peek(), Some(&2.));

        // Pop still OK
        assert_eq!(list.pop(), Some(2.));
        assert_eq!(list.pop(), Some(1.));

        list.push(3.);
        list.push(4.);

        // Once you pop, you just can't stop.
        assert_eq!(list.pop(), Some(4.));
        assert_eq!(list.pop(), Some(3.));
        assert_eq!(list.pop(), Some(0.));
        assert_eq!(list.pop(), None);
    }
}