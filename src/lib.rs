//! Module for stacking line-drawing characters on top of each other.
//!
//! # Examples
//!
//! ```
//! let combo = unicode_line_stacker::stack('┌', '┴');
//! assert_eq!(Some('┼'), combo);
//!
//! // Bit string format: for each of the four directions, clockwise starting from
//! // top (least significant to most significant), 1 means "on" and 0 means "off."
//! let c = unicode_line_stacker::bits_to_char(0b1011);
//! assert_eq!('┴', c);
//! ```

/// Stack two line-drawing characters on top of each other and return the result.
///
/// Returns `None` if one or both of the input characters are unsupported.
///
/// # Examples
///
/// ```
/// let a = '─';
/// let b = '│';
/// let result = unicode_line_stacker::stack(a, b);
/// assert_eq!(Some('┼'), result);
/// ```
pub fn stack(a: char, b: char) -> Option<char> {
    let look_for_a = char_to_bits(a);
    let look_for_b = char_to_bits(b);

    if look_for_b.is_none() || look_for_b.is_none() {
        None
    } else {
        Some(LINE_DRAWING_CHARS[look_for_a.unwrap() | look_for_b.unwrap()])
    }
}

/// Convert a line-drawing char to a bitset (or None if the char is unsupported).
///
/// # Examples
///
/// ```
/// let c = '┬';
/// let result = unicode_line_stacker::char_to_bits(c);
/// assert_eq!(Some(0b1110), result);
/// ```
#[inline]
pub fn char_to_bits(c: char) -> Option<usize> {
    // It's likely faster to iterate through 16 chars than it is to try some
    // HashMap trickery.
    LINE_DRAWING_CHARS
        .iter()
        .enumerate()
        .find(|&(_, &c2)| c == c2)
        .map(|tup| tup.0)
}

/// Convert a bitset to a line-drawing char.
///
/// This crate's representation of each line-drawing char is a `u32`
/// representing a bitset: starting from least significant bit, the bits
/// represent up, right, down, left, in that order.
///
/// # Examples
///
/// ```
/// assert_eq!('┤', unicode_line_stacker::bits_to_char(0b1101));
/// ```
///
/// # Panics
///
/// Panics if `bits >= 16`.
#[inline]
pub fn bits_to_char(bits: u32) -> char {
    if bits >= 16 {
        panic!(
            "Bit set must be between 0 and 15 inclusive but got {}",
            bits
        );
    }

    LINE_DRAWING_CHARS[bits as usize]
}

const LINE_DRAWING_CHARS: [char; 16] = [
    ' ',        // 0000
    '\u{2575}', // 0001
    '\u{2576}', // 0010
    '\u{2514}', // 0011
    '\u{2577}', // 0100
    '\u{2502}', // 0101
    '\u{250c}', // 0110
    '\u{251c}', // 0111
    '\u{2574}', // 1000
    '\u{2518}', // 1001
    '\u{2500}', // 1010
    '\u{2534}', // 1011
    '\u{2510}', // 1100
    '\u{2524}', // 1101
    '\u{252c}', // 1110
    '\u{253c}', // 1111
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_with_empty() {
        let base_char = '\u{2514}'; // light up and right

        let result = stack(' ', base_char);

        assert_eq!(Some(base_char), result);
    }

    #[test]
    #[should_panic(expected = "but got 16")]
    fn bits_to_char_panics_on_input_16() {
        bits_to_char(16);
    }
}
