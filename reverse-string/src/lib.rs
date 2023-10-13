extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // https://stackoverflow.com/a/27996791
    let reversed: String = input
        // Split the string into an Iterator of &strs, where each element is an
        // extended grapheme cluster.
        .graphemes(true)
        // Reverse the order of the grapheme iterator.
        .rev()
        // Collect all the chars into a new owned String.
        .collect();
    return reversed
    // -------
    // 'cargo test' would pass, 'cargo test --features grapheme' would not 
    // let reversed: String = input.chars().rev().collect();
    // return reversed
}
