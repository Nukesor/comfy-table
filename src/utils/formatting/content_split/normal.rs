use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// returns printed length of string
/// if ansi feature enabled, takes into account escape codes
#[inline(always)]
pub fn measure_text_width(s: &str) -> usize {
    s.width()
}

/// Split a line into its individual parts along the given delimiter.
pub fn split_line_by_delimiter(line: &str, delimiter: char) -> Vec<String> {
    line.split(delimiter)
        .map(ToString::to_string)
        .collect::<Vec<String>>()
}

/// Splits a long word at a given character width.
/// This needs some special logic, as we have to take multi-character UTF-8 symbols into account.
/// When simply splitting at a certain char position, we might end up with a string that's has a
/// wider display width than allowed.
pub fn split_long_word(allowed_width: usize, word: &str) -> (String, String) {
    let mut current_width = 0;
    let mut parts = String::new();

    let mut char_iter = word.chars().peekable();
    // Check if the string might be too long, one character at a time.
    // Peek into the next char and check the exit condition.
    // That is, pushing the next character would result in the string being too long.
    while let Some(c) = char_iter.peek() {
        if (current_width + c.width().unwrap_or(1)) > allowed_width {
            break;
        }

        // We can unwrap, as we just checked that a suitable character is next in line.
        let c = char_iter.next().unwrap();

        // We default to 1 char, if the character length cannot be determined.
        // The user has to live with this, if they decide to add control characters or some fancy
        // stuff into their tables. This is considered undefined behavior and we try to handle this
        // to the best of our capabilities.
        let character_width = c.width().unwrap_or(1);

        current_width += character_width;
        parts.push(c);
    }

    // Collect the remaining characters.
    let remaining = char_iter.collect();
    (parts, remaining)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_long_word() {
        let emoji = "🙂‍↕️"; // U+1F642 U+200D U+2195 U+FE0F head shaking vertically
        assert_eq!(emoji.len(), 13);
        assert_eq!(emoji.chars().count(), 4);
        assert_eq!(emoji.width(), 2);

        let (word, remaining) = split_long_word(emoji.width(), &emoji);

        println!("{word}");
        assert_eq!(word, "\u{1F642}\u{200D}");
        assert_eq!(word.len(), 7);
        assert_eq!(word.chars().count(), 2);
        assert_eq!(word.width(), 2);

        println!("{remaining}");
        assert_eq!(remaining, "\u{2195}\u{FE0F}");
        assert_eq!(remaining.len(), 6);
        assert_eq!(remaining.chars().count(), 2);
        assert_eq!(remaining.width(), 2);
    }
}
