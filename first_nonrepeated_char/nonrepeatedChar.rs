use std::collections::hashmap::HashMap;

fn first_nonrepeated_char(input_str: &str) -> Option<char> {
  let mut char_hash:HashMap<char, uint> = HashMap::new();

  for c in input_str.chars() {
    char_hash.insert_or_update_with(c, 1u, |_key, val| *val += 1u);
  }

  for c in input_str.chars() {
    if char_hash[c] == 1u {
      return Some(c);
    }
  }

  None
}

#[test]
fn should_find_middle_char() {
  let s:&str = "ABA";
  assert_eq!(first_nonrepeated_char(s), Some('B'));
}

#[test]
fn should_not_find_nonrepeated_char() {
  let s:&str = "AABBCCDD";
  assert_eq!(first_nonrepeated_char(s), None);
}

#[cfg(not(test))]
fn main() {
}