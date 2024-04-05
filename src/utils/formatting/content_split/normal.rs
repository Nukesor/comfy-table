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
