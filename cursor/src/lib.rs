use im::Vector;
use std::fmt;

pub struct ListCursor<T> {
    previous: Vector<T>,
    next: Vector<T>,
}

impl<T> ListCursor<T>
where
    T: Clone,
{
    pub fn new(list: Vector<T>) -> Self {
        Self {
            previous: Vector::new(),
            next: list,
        }
    }

    pub fn rebuild(self) -> Vector<T> {
        let Self { previous, mut next } = self;
        next.extend(previous.into_iter().rev());
        next
    }
}

impl<T: fmt::Debug + Clone> fmt::Debug for ListCursor<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ListCursor {{previous: {:?}, next: {:?}}}",
            self.previous, self.next
        )
    }
}

impl<T> PartialEq for ListCursor<T>
where
    T: PartialEq + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.previous == other.previous && self.next == other.next
    }
}

impl<T> ListCursor<T>
where
    T: Clone,
{
    pub fn next(mut self) -> Option<Self> {
        match self.next.pop_front() {
            None => None,
            Some(t) => {
                let Self { mut previous, next } = self;
                previous.push_front(t);
                Some(Self { previous, next })
            }
        }
    }

    pub fn prev(mut self) -> Option<Self> {
        match self.previous.pop_front() {
            None => None,
            Some(t) => {
                let Self { previous, mut next } = self;
                next.push_front(t);
                Some(Self { previous, next })
            }
        }
    }
}
#[cfg(test)]
mod tests {

    use im::vector;

    use super::*;

    #[test]
    fn can_rebuild_vec_from_cursor() {
        let expected = vector![1, 2, 3];
        let cursor = ListCursor::new(expected.clone());
        assert_eq!(cursor.rebuild(), expected);
    }

    #[test]
    fn next_works() {
        let cursor = ListCursor::new(vector![1, 2, 3]);

        let cursor1 = cursor.next().unwrap();
        let expected1 = ListCursor {
            previous: vector![1],
            next: vector![2, 3],
        };
        assert_eq!(cursor1, expected1);

        let cursor2 = cursor1.next().unwrap();
        let expected2 = ListCursor {
            previous: vector![2, 1],
            next: vector![3],
        };
        assert_eq!(cursor2, expected2);

        let cursor3 = cursor2.next().unwrap();
        let expected3 = ListCursor {
            previous: vector![3, 2, 1],
            next: vector![],
        };
        assert_eq!(cursor3, expected3);

        assert_eq!(cursor3.next(), None);
    }

    #[test]
    fn prev_works() {
        let cursor = ListCursor {
            previous: vector![1, 2, 3],
            next: vector![],
        };

        let cursor1 = cursor.prev().unwrap();
        let expected1 = ListCursor {
            previous: vector![2, 3],
            next: vector![1],
        };
        assert_eq!(cursor1, expected1);

        let cursor2 = cursor1.prev().unwrap();
        let expected2 = ListCursor {
            previous: vector![3],
            next: vector![2, 1],
        };
        assert_eq!(cursor2, expected2);

        let cursor3 = cursor2.prev().unwrap();
        let expected3 = ListCursor {
            previous: vector![],
            next: vector![3, 2, 1],
        };
        assert_eq!(cursor3, expected3);

        assert_eq!(cursor3.prev(), None);
    }
}
