pub fn string_to_digits(data: String) -> Vec<u32> {
  data
    .chars()
    .filter_map(|c| c.to_digit(10))
    .collect()
}
