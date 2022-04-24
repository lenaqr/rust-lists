pub struct List<'a, T> {
    pub data: T,
    pub next: Option<&'a List<'a, T>>,
}

impl<'a, T> List<'a, T> {
    pub fn prepend(next: Option<&'a List<'a, T>>, data: T) -> Self {
        List { data, next }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a List<'a, T>>,
}

impl<'a, T> List<'a, T> {
    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter { next: Some(self) }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next;
            &node.data
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::prepend(None, 3);
        assert_eq!(list.iter().copied().sum::<i32>(), 3);

        let list = List::prepend(Some(&list), 5);
        assert_eq!(list.iter().copied().sum::<i32>(), 5 + 3);

        let list = List::prepend(Some(&list), 13);
        assert_eq!(list.iter().copied().sum::<i32>(), 13 + 5 + 3);
    }

    #[test]
    fn stacked() {
        let list = List::prepend(None, 3);
        assert_eq!(list.iter().copied().sum::<i32>(), 3);

        let list = List::prepend(Some(&list), 5);
        assert_eq!(list.iter().copied().sum::<i32>(), 5 + 3);

        {
            let list = List::prepend(Some(&list), 8);
            assert_eq!(list.iter().copied().sum::<i32>(), 8 + 5 + 3);
        }
        {
            let list = List::prepend(Some(&list), 13);
            assert_eq!(list.iter().copied().sum::<i32>(), 13 + 5 + 3);
        }
    }
}
