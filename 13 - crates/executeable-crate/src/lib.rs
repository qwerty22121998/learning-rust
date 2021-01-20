pub fn sum(a: i64, b: i64) -> i64 {
  a + b
}

pub fn hello() -> String {
  "it's me dio".to_string()
}

#[cfg(test)]
mod tests {
  use super::hello;

  #[test]
  fn test_hello() {
    assert_eq!("it's me dio", hello())
  }
}
