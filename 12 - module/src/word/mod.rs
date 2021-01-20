mod vietnam;

pub fn hello_word() -> String {
  "Hello world".to_string()
}

pub use self::vietnamese::capital; //re export so we can use vietnam::capital()

pub mod vietnamese {

  use super::vietnam;

  pub fn flirt() -> String {
    "em ăn cơm chưa?".to_string()
  }

  pub fn capital() -> String {
    vietnam::capital()
  }
}
