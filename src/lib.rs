pub fn series(digits: &str, len: usize) -> Vec<String> {
  if len > digits.len() {
    return Vec::new();
  }
  if len == digits.len() {
    return vec![String::from(digits)];
  }
  let mut res: Vec<String> = Vec::new();
  for start in 0..usize::max_value() {
    match digits.get(start..(len + start)) {
      Some(str) => res.push(String::from(str)),
      None => break,
    }
  }
  res
}
