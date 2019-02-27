pub fn series(digits: &str, len: usize) -> Vec<String> {
  return match (len, digits.len()) {
    (x, y) if x > y => vec![],
    (x, y) if x == y => vec![digits.to_string()],
    _ => find_substrings(digits, len),
  };
}

pub fn find_substrings(digits: &str, substr_length: usize) -> Vec<String> {
  (0..(digits.len() - substr_length + 1))
    .filter_map(|start| digits.get(start..(substr_length + start)))
    .map(|str| str.to_string())
    .collect()
}
