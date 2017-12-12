use std::str::SplitWhitespace;
use std::collections::HashSet;

/// Count of passphrases with distinct words
pub fn part_1(data: String) -> String {
  data.lines()
    .map(|line| line.split_whitespace())
    .filter(|phrase| is_valid_part_1(phrase))
    .count()
    .to_string()
}

/// Count of passphrases with distinct sorted (anagram) words
pub fn part_2(data: String) -> String {
  data.lines()
    .map(|line| line.split_whitespace())
    .filter(|phrase| is_valid_part_2(phrase))
    .count()
    .to_string()
}

fn is_valid_part_1(words: &SplitWhitespace) -> bool {
  let mut set = HashSet::new();
  for word in words.clone() {
    if !set.insert(word) { return false };
  }
  true
}

fn is_valid_part_2(words: &SplitWhitespace) -> bool {
  let mut set = HashSet::new();
  for word in words.clone() {
    let mut sorted_word = word.chars().collect::<Vec<char>>();
    sorted_word.sort();
    if !set.insert(sorted_word) { return false };
  }
  true
}
