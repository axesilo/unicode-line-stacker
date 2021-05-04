# Unicode Line Stacker

## What is this crate?

Suppose you are drawing "lines" in the terminal using
[box-drawing characters](https://en.wikipedia.org/wiki/Box-drawing_character) and are filling in a 2-D grid of `char`s.

You might first put down a ┌ but then later want to draw a ┴ on top of that.  This crate will calculate that the resulting char should be a ┼.

You can also convert a 4-bit bit string into a line drawing character.

## Usage

```rust
let combo = unicode_line_stacker::stack('┌', '┴');
assert_eq!(Some('┼'), combo);

// Bit string format: for each of the four directions, clockwise starting from
// top (least significant to most significant), 1 means "on" and 0 means "off."
let c = unicode_line_stacker::bits_to_char(0b1011);
assert_eq!('┴', c);
```

## Current Functionality

Right now the crate only supports the "light" line drawing characters in the four cardinal directions.