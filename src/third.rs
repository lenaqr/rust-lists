use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        let next = self.head.clone();
        let head = Some(Rc::new(Node { elem, next }));
        List { head }
    }

    pub fn headtail(&self) -> Option<(&T, List<T>)> {
        let Node { elem, next } = self.head.as_deref()?;
        let tail = List { head: next.clone() };
        Some((elem, tail))
    }

    pub fn iter(&self) -> Iter<'_, T> {
        let next = self.head.as_deref();
        Iter { next }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(Ok(node)) = head.map(Rc::try_unwrap) {
            head = node.next;
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let Node { elem, next } = self.next?;
        self.next = next.as_deref();
        Some(&elem)
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert!(list.headtail().is_none());

        let list = list.prepend(1).prepend(2).prepend(3);

        let (head, list) = list.headtail().unwrap();
        assert_eq!(head, &3);

        let (head, list) = list.headtail().unwrap();
        assert_eq!(head, &2);

        let (head, list) = list.headtail().unwrap();
        assert_eq!(head, &1);

        assert!(list.headtail().is_none());
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn drop_no_stack_overflow() {
        let mut list = List::new();
        for i in 1..20000 {
            list = list.prepend(i);
        }
    }
}
