pub fn from_string(data: String) -> Self {
  let mut words = BTreeSet::new();
  let (ptr, len, cap) = data.into_raw_parts();

  let data = unsafe {
      let slice = std::slice::from_raw_parts(ptr, len);
      let pool: &'static str = std::str::from_utf8_unchecked(slice);
      'outer: for word in pool.lines() {
          let mut count = 0;
          for char in word.chars() {
              if !(char.is_ascii_alphanumeric() && char.is_ascii_lowercase()) {
                  continue 'outer;
              }
              count += 1;
              if count == 6 {
                  continue 'outer;
              }
          }
          if count == 5 {
              words.insert(word);
          }
      }
      String::from_raw_parts(ptr, len, cap)
  };

  Self { data, words }
}