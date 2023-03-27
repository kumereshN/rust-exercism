use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let reversed_string = UnicodeSegmentation::graphemes(input, true).rev().collect();
    reversed_string
}
