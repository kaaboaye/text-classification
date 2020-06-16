#[derive(Debug)]
pub struct CountedWord<'a> {
  pub word: &'a str,
  pub count: u32,
}

impl<'a> PartialEq for CountedWord<'a> {
  fn eq(&self, rhs: &Self) -> bool {
    self.count == rhs.count
  }
}

impl<'a> Eq for CountedWord<'a> {}

impl<'a> PartialOrd for CountedWord<'a> {
  fn partial_cmp(&self, rhs: &Self) -> std::option::Option<std::cmp::Ordering> {
    self.count.partial_cmp(&rhs.count)
  }
}

impl<'a> Ord for CountedWord<'a> {
  fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
    self.count.cmp(&rhs.count)
  }
}
