use utils::string_to_digits;

/// Sum of all the digits in a circular list that are equal to their neighbor
pub fn part_1(data: String) -> String {
  let mut digits = string_to_digits(data);

  let first = digits[0];

  digits.push(first);
  let mut previous = 99;
  let mut result = 0;
  for digit in digits {
    if digit == previous {
      result += digit;
    }
    previous = digit;
  }
  result.to_string()
}

/// Sum of all the digits in a circular list that are equal to the digit halfway apart
pub fn part_2(data: String) -> String {
  let digits = string_to_digits(data);
  let length = digits.len();
  let mut result = 0;

  let compare_to = |i| (i + length / 2) % length;

  for i in 0..length {
    if digits[i] == digits[compare_to(i)] {
      result += digits[i];
    }
  }

  result.to_string()
}
