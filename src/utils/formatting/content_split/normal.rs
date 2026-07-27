use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Returns printed length of string
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
    // Most input is usually normal ASCII, which makes UTF-8 grapheme aware splitting
    // unnecessary. Handling this fast path speeds up splitting significantly.
    //
    // We do a bit of extra checking on the `<0x20` part so that we don't skip control
    // chars. Not sure what would happen if there was a `\r` is in there.
    if word.bytes().all(|byte| (0x20..0x7f).contains(&byte)) {
        let split_index = word.len().min(allowed_width);
        let (parts, remaining) = word.split_at(split_index);
        return (parts.to_string(), remaining.to_string());
    }

    let mut current_width = 0;
    let mut split_index = word.len();

    // Check if the string might be too long, one Unicode grapheme at a time.
    //
    // This code uses graphemes to handle both zero-width joiner[0] UTF-8 chars, which
    // combine multiple UTF-8 chars into a single grapheme, and variant selectors [1],
    // which pick a certain variant of the preceding char.
    //
    // [0]: https://en.wikipedia.org/wiki/Zero-width_joiner
    // [1]: https://en.wikipedia.org/wiki/Variation_Selectors_(Unicode_block)
    for (index, c) in word.grapheme_indices(true) {
        let character_width = c.width();
        if (current_width + character_width) > allowed_width {
            split_index = index;
            break;
        }

        current_width += character_width;
    }

    let (parts, remaining) = word.split_at(split_index);
    (parts.to_string(), remaining.to_string())
}
