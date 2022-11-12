use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::utils::ColumnDisplayInfo;

/// returns printed length of string
/// if ansi feature enabled, takes into account escape codes
pub fn measure_text_width(s: &str) -> usize {
    #[cfg(feature = "ansi")]
    let width = console::measure_text_width(s);

    #[cfg(not(feature = "ansi"))]
    let width = s.width();
    width
}

/// Split the line by the given deliminator without breaking ansi codes that contain the delimiter
#[cfg(feature = "ansi")]
pub fn ansi_aware_split(line: &str, delimiter: char) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut current_line = String::default();

    // Iterate over line, spliting text with delimiter
    let iter = console::AnsiCodeIterator::new(line);
    for (str_slice, is_esc) in iter {
        if is_esc {
            current_line.push_str(str_slice);
        } else {
            let mut split = str_slice.split(delimiter);

            // Text before first delimiter (if any) belongs to previous line
            let first = split
                .next()
                .expect("split always produces at least one value");
            current_line.push_str(first);

            // Text after each delimiter goes to new line.
            for text in split {
                lines.push(current_line);
                current_line = text.to_string();
            }
        }
    }
    lines.push(current_line);
    fix_style_in_split_str(lines.as_mut());
    lines
}

/// Split a line if it's longer than the allowed columns (width - padding).
///
/// This function tries to do this in a smart way, by splitting the content
/// with a given delimiter at the very beginning.
/// These "elements" then get added one-by-one to the lines, until a line is full.
/// As soon as the line is full, we add it to the result set and start a new line.
///
/// This is repeated until there're no more "elements".
///
/// Mid-element splits only occurs if a element doesn't fit in a single line by itself.
pub fn split_line(line: &str, info: &ColumnDisplayInfo, delimiter: char) -> Vec<String> {
    let mut lines = Vec::new();
    let content_width = usize::from(info.content_width);

    // Split the line by the given deliminator and turn the content into a stack.
    // Also clone it and convert it into a Vec<String>. Otherwise we get some burrowing problems
    // due to early drops of borrowed values that need to be inserted into `Vec<&str>`
    #[cfg(not(feature = "ansi"))]
    let mut elements = line
        .split(delimiter)
        .map(ToString::to_string)
        .collect::<Vec<String>>();

    #[cfg(feature = "ansi")]
    let mut elements = ansi_aware_split(line, delimiter);

    // Reverse it, since we want to push/pop without reversing the text.
    elements.reverse();

    let mut current_line = String::new();
    while let Some(next) = elements.pop() {
        let current_length = measure_text_width(&current_line);
        let next_length = measure_text_width(&next);

        // Some helper variables
        // The length of the current line when combining it with the next element
        // Add 1 for the delimiter if we are on a non-emtpy line.
        let mut added_length = next_length + current_length;
        if !current_line.is_empty() {
            added_length += 1;
        }
        // The remaining width for this column. If we are on a non-empty line, subtract 1 for the delimiter.
        let mut remaining_width = content_width - current_length;
        if !current_line.is_empty() {
            remaining_width = remaining_width.saturating_sub(1);
        }

        // The next element fits into the current line
        if added_length <= content_width {
            // Only add delimiter, if we're not on a fresh line
            if !current_line.is_empty() {
                current_line.push(delimiter);
            }
            current_line += &next;

            // Already complete the current line, if there isn't space for more than two chars
            current_line = check_if_full(&mut lines, content_width, current_line);
            continue;
        }

        // The next element doesn't fit in the current line

        // Check, if there's enough space in the current line in case we decide to split the
        // element and only append a part of it to the current line.
        // If there isn't enough space, we simply push the current line, put the element back
        // on stack and start with a fresh line.
        if !current_line.is_empty() && remaining_width <= MIN_FREE_CHARS {
            elements.push(next);
            lines.push(current_line);
            current_line = String::new();

            continue;
        }

        // Ok. There's still enough space to fit something in (more than MIN_FREE_CHARS characters)
        // There are two scenarios:
        //
        // 1. The word is too long for a single line.
        //    In this case, we have to split the element anyways. Let's fill the remaining space on
        //    the current line with, start a new line and push the remaining part on the stack.
        // 2. The word is short enough to fit as a whole into a line
        //    In that case we simply push the current line and start a new one with the current element

        // Case 1
        // The element is longer than the specified content_width
        // Split the word, push the remaining string back on the stack
        if next_length > content_width {
            let new_line = current_line.is_empty();

            // Only add delimiter, if we're not on a fresh line
            if !new_line {
                current_line.push(delimiter);
                //remaining_width = remaining_width.saturating_sub(1);
            }

            let (mut next, mut remaining) = split_long_word(remaining_width, &next);

            // TODO: This is a pretty hefty hack, but it's needed for now.
            //
            // Scenario: We try to split a word that doesn't fit into the current line.
            // It's supposed to be a new line, with a width of 1. However, the next char in line
            // is a multi-character UTF-8 symbol.
            //
            // Since a, for instance, two-character wide symbol doesn't fit into a 1-character
            // column, this code would loop endlessly. (There's no legitimate way to split that
            // word.)
            // Hence, we have to live with the fact, that this line will look broken, as we put a
            // two-character wide symbol into it.
            if new_line && next.is_empty() {
                let mut chars = remaining.chars();
                next.push(chars.next().unwrap());
                remaining = chars.collect();
            }

            current_line += &next;
            elements.push(remaining);

            // Push the finished line, and start a new one
            lines.push(current_line);
            current_line = String::new();

            continue;
        }

        // Case 2
        // The element fits into a single line.
        // Push the current line and initialize the next line with the element.
        lines.push(current_line);
        current_line = next.to_string();
        current_line = check_if_full(&mut lines, content_width, current_line);
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

/// This is the minimum of available characters per line.
/// It's used to check, whether another element can be added to the current line.
/// Otherwise the line will simply be left as it is and we start with a new one.
/// Two chars seems like a reasonable approach, since this would require next element to be
/// a single char + delimiter.
const MIN_FREE_CHARS: usize = 2;

/// Check if the current line is too long and whether we should start a new one
/// If it's too long, we add the current line to the list of lines and return a new [String].
/// Otherwise, we simply return the current line and basically don't do anything.
fn check_if_full(lines: &mut Vec<String>, content_width: usize, current_line: String) -> String {
    // Already complete the current line, if there isn't space for more than two chars
    if measure_text_width(&current_line) > content_width.saturating_sub(MIN_FREE_CHARS) {
        lines.push(current_line);
        return String::new();
    }

    current_line
}

#[cfg(feature = "ansi")]
const ANSI_RESET: &str = "\u{1b}[0m";

/// Splits a long word at a given character width.
/// This needs some special logic, as we have to take multi-character UTF-8 symbols into account.
/// When simply splitting at a certain char position, we might end up with a string that's has a
/// wider display width than allowed.
#[cfg(not(feature = "ansi"))]
fn split_long_word(allowed_width: usize, word: &str) -> (String, String) {
    let mut current_width = 0;
    let mut splitted = String::new();

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
        splitted.push(c);
    }

    // Collect the remaining characters.
    let remaining = char_iter.collect();
    (splitted, remaining)
}

/// Splits a long word at a given character width. Inserting the needed ansi codes to preserve style.
#[cfg(feature = "ansi")]
fn split_long_word(allowed_width: usize, word: &str) -> (String, String) {
    // A buffer for the first half of the split str, which will take up at most `allowed_len` characters when printed to the terminal.
    let mut head = String::with_capacity(word.len());
    // A buffer for the second half of the split str
    let mut tail = String::with_capacity(word.len());
    // Tracks the len() of head
    let mut head_len = 0;
    // Tracks the len() of head, sans trailing ansi escape codes
    let mut head_len_last = 0;
    // Count of *non-trailing* escape codes in the buffer.
    let mut escape_count_last = 0;
    // A buffer for the escape codes that exist in the str before the split.
    let mut escapes = Vec::new();

    // Iterate over segments of the input string, each segment is either a singe escape code or block of text containing no escape codes.
    // Add text and escape codes to the head buffer, keeping track of printable length and what ansi codes are active, untill there is no more room in allowed_width.
    // If the str was split at a point with active escape-codes, add the ansi reset code to the end of head, and the list of active escape codes to the beginning of tail.
    let mut iter = console::AnsiCodeIterator::new(word);
    for (str_slice, is_esc) in iter.by_ref() {
        if is_esc {
            escapes.push(str_slice);
            // If the code is reset, that means all current codes in the buffer can be ignored.
            if str_slice == ANSI_RESET {
                escapes.clear();
            }
        }

        let slice_len = match is_esc {
            true => 0,
            false => str_slice.width(),
        };

        if head_len + slice_len <= allowed_width {
            head.push_str(str_slice);
            head_len += slice_len;

            if !is_esc {
                // allows popping unneeded escape codes later
                head_len_last = head.len();
                escape_count_last = escapes.len();
            }
        } else {
            assert!(!is_esc);
            let mut char_iter = str_slice.chars().peekable();
            while let Some(c) = char_iter.peek() {
                let character_width = c.width().unwrap_or(0);
                if allowed_width < head_len + character_width {
                    break;
                }

                head_len += character_width;
                let c = char_iter.next().unwrap();
                head.push(c);

                // c is not escape code
                head_len_last = head.len();
                escape_count_last = escapes.len();
            }

            // cut off dangling escape codes since they should have no effect
            head.truncate(head_len_last);
            if escape_count_last != 0 {
                head.push_str(ANSI_RESET);
            }

            for esc in escapes {
                tail.push_str(esc);
            }
            let remaining: String = char_iter.collect();
            tail.push_str(&remaining);
            break;
        }
    }

    iter.for_each(|s| tail.push_str(s.0));
    (head, tail)
}

/// Fixes ansi escape codes in a split string
/// 1. Adds reset code to the end of each substring if needed.
/// 2. Keeps track of previous substring's escape codes and inserts them in later substrings to continue style
#[cfg(feature = "ansi")]
pub fn fix_style_in_split_str(words: &mut [String]) {
    let mut escapes: Vec<String> = Vec::new();

    for word in words {
        // before we modify the escape list, make a copy
        let prepend = if !escapes.is_empty() {
            Some(escapes.join(""))
        } else {
            None
        };

        // add escapes in word to escape list
        let iter = console::AnsiCodeIterator::new(word)
            .filter(|(_, is_esc)| *is_esc)
            .map(|v| v.0);
        for esc in iter {
            if esc == ANSI_RESET {
                escapes.clear()
            } else {
                escapes.push(esc.to_string())
            }
        }

        // insert previous esc sequences at the beginning of the segment
        if let Some(prepend) = prepend {
            word.insert_str(0, &prepend);
        }

        // if there are active escape sequences, we need to append reset
        if !escapes.is_empty() {
            word.push_str(ANSI_RESET);
        }
    }
}

#[cfg(test)]

mod test {
    #[cfg(feature = "ansi")]
    #[test]
    fn ansi_aware_split_test() {
        use super::ansi_aware_split;

        let text = "\u{1b}[1m head [ middle [ tail \u{1b}[0m[ after";
        let split = ansi_aware_split(text, '[');

        assert_eq!(
            split,
            [
                "\u{1b}[1m head \u{1b}[0m",
                "\u{1b}[1m middle \u{1b}[0m",
                "\u{1b}[1m tail \u{1b}[0m",
                " after"
            ]
        )
    }
}
