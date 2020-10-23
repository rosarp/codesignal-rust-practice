use std::collections::HashMap;
use std::collections::HashSet;

fn first_not_repeating_character(input_string: String) -> char {
    let mut duplicate_chars : HashSet<char> = HashSet::new();
    let mut chars_index : HashMap<char, usize> = HashMap::new();
    for (i, ch) in input_string.chars().enumerate() {
        if chars_index.contains_key(&ch) {
            duplicate_chars.insert(ch);
            chars_index.remove(&ch);
        } else if !duplicate_chars.contains(&ch) {
            chars_index.insert(ch, i);
        }
    }
    let mut char_index = ('_', input_string.len());
    for (ch, idx) in &chars_index {
        if idx < &char_index.1 {
            char_index = (*ch, *idx)
        }
    }
    char_index.0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_first_not_repeating_character() {
    // 1
    let mut s = first_not_repeating_character(String::from("abacabad"));
    assert_eq!(s, 'c');
    // 2
    s = first_not_repeating_character(String::from("abacabaabacaba"));
    assert_eq!(s, '_');
    // 3
    s = first_not_repeating_character(String::from("z"));
    assert_eq!(s, 'z');
    // 4
    s = first_not_repeating_character(String::from("bcb"));
    assert_eq!(s, 'c');
    // 5
    s = first_not_repeating_character(String::from("bcccccccb"));
    assert_eq!(s, '_');
    // 6
    s = first_not_repeating_character(String::from("abcdefghijklmnopqrstuvwxyziflskecznslkjfabe"));
    assert_eq!(s, 'd');
    // 7
    s = first_not_repeating_character(String::from("zzz"));
    assert_eq!(s, '_');
    // 8
    s = first_not_repeating_character(String::from("bcccccccccccccyb"));
    assert_eq!(s, 'y');
    // 9
    s = first_not_repeating_character(String::from("xdnxxlvupzuwgigeqjggosgljuhliybkjpibyatofcjbfxwtalc"));
    assert_eq!(s, 'd');
    // 10
    s = first_not_repeating_character(String::from("ngrhhqbhnsipkcoqjyviikvxbxyphsnjpdxkhtadltsuxbfbrkof"));
    assert_eq!(s, 'g');
  }
}