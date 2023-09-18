use persi_ds::sync::{
    list::{concat, reverse},
    List,
};
use std::fmt;

// #[derive(PartialEq)]
pub struct ListCursor<T> {
    previous: List<T>,
    next: List<T>,
}

impl<T> Default for ListCursor<T> {
    fn default() -> Self {
        Self {
            previous: List::default(),
            next: List::default(),
        }
    }
}
impl<T> PartialEq for ListCursor<T>
where
    T: PartialEq + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        &self.previous == &other.previous && &self.next == &other.next
    }
}
impl<T> ListCursor<T> {
    pub fn new(list: List<T>) -> Self {
        Self {
            previous: List::new(),
            next: list,
        }
    }

    pub fn rebuild(&self) -> List<T>
    where
        T: Clone,
    {
        // let Self { previous, mut next } = self;
        // next.extend(previous.into_iter().rev());
        concat(&reverse(self.previous.clone()), &self.next)
    }
}

impl<T: fmt::Debug> fmt::Debug for ListCursor<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ListCursor {{previous: {:?}, next: {:?}}}",
            self.previous, self.next
        )
    }
}

impl<T: Clone> ListCursor<T> {
    pub fn next(&self) -> Option<ListCursor<T>> {
        if self.next.is_empty() {
            return None;
        }
        match self.next.head_tail() {
            (None, _) => None,
            (Some(head), tail) => Some(ListCursor {
                previous: self.previous.pushed_front(head),
                next: tail,
            }),
        }
    }

    pub fn prev(&self) -> Option<ListCursor<T>> {
        if self.previous.is_empty() {
            return None;
        }
        match self.previous.head_tail() {
            (None, _) => None,
            (Some(head), tail) => Some(ListCursor {
                previous: tail,
                next: self.next.pushed_front(head),
            }),
        }
    }

    pub fn insert(&self, t: T) -> Self {
        Self {
            previous: self.previous.pushed_front(t),
            next: self.next.clone(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use persi_ds::synced_list;

    #[test]
    fn can_rebuild_vec_from_cursor() {
        let cursor = ListCursor {
            previous: synced_list![1, 2],
            next: synced_list![4, 3],
        };
        let expected = synced_list![4, 3, 2, 1];
        assert_eq!(cursor.rebuild(), expected);
    }

    #[test]
    fn next_and_prev() {
        let cursor = ListCursor::new(synced_list![3, 2, 1]);

        let cursor1 = cursor.next().unwrap();

        let expected1 = ListCursor {
            previous: synced_list![1],
            next: synced_list![3, 2],
        };

        assert_eq!(cursor1, expected1);

        let cursor2 = cursor1.next().unwrap();

        let expected2 = ListCursor {
            previous: synced_list![1, 2],
            next: synced_list!(3),
        };
        assert_eq!(cursor2, expected2);

        let cursor3 = cursor2.next().unwrap();

        let expected3 = ListCursor {
            previous: synced_list!(1, 2, 3),
            next: synced_list!(),
        };
        assert_eq!(cursor3, expected3);

        assert_eq!(cursor3.next(), None);

        let cursor4 = cursor3.prev().unwrap();

        assert_eq!(cursor4, expected2);

        let cursor5 = cursor4.prev().unwrap();

        assert_eq!(cursor5, expected1);

        let cursor6 = cursor5.prev().unwrap();

        assert_eq!(cursor6, cursor);

        assert_eq!(cursor6.prev(), None);
    }

    #[test]
    fn can_insert() {
        let lc = ListCursor::default();
        let lc1 = lc.insert(1);

        let expected = ListCursor {
            previous: synced_list!(1),
            ..Default::default()
        };
        assert_eq!(lc1, expected);
    }
}
