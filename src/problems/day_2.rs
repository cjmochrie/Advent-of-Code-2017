use std::iter::Iterator;

/// Break newline separated text into rows and sum the difference
/// between the max and min in each row
pub fn part_1(data: String) -> String {
  string_to_digit_rows(&data)
    .map(|mut row: Vec<u32>| {
      row.sort();
      row.last().unwrap() - row.first().unwrap()
    })
    .fold(0, |sum, i| sum + i)
    .to_string()
}

/// Same data transformation, but now find the two numbers
/// in a row that evenly dived. Sum the result of dividing them
pub fn part_2(data: String) -> String {
  string_to_digit_rows(&data)
    .map(process_row)
    .fold(0, |sum, i| sum + i).to_string()
}

fn string_to_digit_rows<'a>(data: &'a str) -> Box<Iterator<Item=Vec<u32>> + 'a> {
  Box::new(data.lines()
    .map(|row| row.split_whitespace().map(|word| word.parse::<u32>().unwrap()).collect()))
}

fn process_row(mut row: Vec<u32>) -> u32 {
  row.sort();
  for i in 0..row.len() {
    for j in (i + 1)..row.len() {
      if row[j] % row[i] == 0 {
        return row[j] / row[i];
      }
    }
  }
  0
}
