use crate::ListCursor;

pub struct TextCursor(ListCursor<char>);

impl Default for TextCursor {
    fn default() -> Self {
        Self(ListCursor::default())
    }
}

impl TextCursor {
    // pub fn new(lc: ListCursor<char>) -> Option<Self> {

    // }
}
impl TextCursor {
    pub fn insert(&self, c: char) -> Option<Self> {
        match c {
            '\n' => None,
            c => Some(Self(self.0.insert(c))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_valid_char_succeeds() {
        let tc = TextCursor::default();
        let tc1 = tc.insert('a');
        assert!(tc1.is_some());
    }

    #[test]
    fn insert_invalid_char_fails() {
        let tc = TextCursor::default();
        let tc1 = tc.insert('\n');
        assert!(tc1.is_none());
    }
}
