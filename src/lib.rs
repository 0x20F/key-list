//! A minimalistic crate to help you quickly
//! get substrings of text between 2 characters
//! inside a string.
#![warn(missing_docs)]


/// The list itself
pub struct KeyList<'a> {
    input: &'a str,
    start: char,
    end: char
}


impl<'a> KeyList<'a> {
    /// Creates a new iterable list of keys
    ///
    /// # Example
    /// ```
    /// # use crate::key_list::KeyList;
    ///
    /// let list = KeyList::new("<a> <b> <c>", '<', '>');
    /// ```
    pub fn new(input: &'a str, start: char, end: char) -> Self {
        Self {
            input,
            start,
            end
        }
    }
}


impl<'a> Iterator for KeyList<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut key = None;

        if let Some(i) = self.input.find(self.start) {
            self.input = &self.input[i..];
            let mut first = true;

            let rest = self.input.char_indices()
                .take_while(|(_, c)| {
                    if first {
                        first = false;
                        return true;
                    }

                    *c != self.end
                })
                .last()
                .map(|(idx, c)| idx + c.len_utf8())
                .unwrap_or_default();

            // +1 to get the end symbol that's excluded
            key = Some(&self.input[..(rest + 1)]);
            self.input = &self.input[(rest + 1)..];
        }

        key
    }
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_keys() {
        let input = "<black> <red> one two <three>";
        let key_count = KeyList::new(input, '<', '>').count();

        assert_eq!(key_count, 3);
    }

    #[test]
    fn different_start() {
        let input = "<A/ <B> one two <C/";
        let key_count = KeyList::new(input, '<', '/').count();

        assert_eq!(key_count, 2);
    }

    #[test]
    fn different_character() {
        let input = "/A/ /B/ one two /C/";
        let key_count = KeyList::new(input, '/', '/').count();

        assert_eq!(key_count, 3);
    }
}