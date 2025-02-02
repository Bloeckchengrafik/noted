/// The Skipmap is a list of indices where single characters are removed from the original text.
/// This is used to map the shaped text back to the original text.
/// For example, if you'd want to figure out the skip offset for the 5th shaped character, you'd
/// count all the skipmap entries that are less or equal than 5.
///
/// Example:
/// Original text:
/// "Hi *Dave*!" ->
/// "Hi Dave!" (see how the '*' characters are removed)
///
/// Skipmap: [4, 9]
///
/// So to get the original text index for the 5th shaped character:
/// 5 + count_skipmap_entries(5) = 5 + 1 = 6
///
/// Or for the 8th shaped character:
/// 8 + count_skipmap_entries(8) = 8 + 1 + count_skipmap_entries(4, 9) = 9 + 1 = 10
///
/// As you can see, when you reach a skipmap entry, you increment the skipmap limit by 1.
pub struct SkipMap {
  map: Vec<usize>,
}

impl SkipMap {
  pub fn new() -> Self {
    Self { map: Vec::new() }
  }

  pub fn add(&mut self, index: usize) {
    self.map.push(index);
  }

  pub fn count_skipmap_entries(&self, mut bound: usize) -> usize {
    let mut count = 0;
    for &skip in &self.map {
      if skip <= bound {
        count += 1;
        bound += 1;
      } else {
        break;
      }
    }

    count
  }

  pub fn original_index(&self, index: usize) -> usize {
    index + self.count_skipmap_entries(index)
  }
}

#[cfg(test)]
mod tests {
  use super::SkipMap;

  #[test]
  fn test_skipmap_new() {
    let skipmap = SkipMap::new();
    assert!(skipmap.map.is_empty());
  }

  #[test]
  fn test_add_to_skipmap() {
    let mut skipmap = SkipMap::new();
    skipmap.add(4);
    skipmap.add(9);
    assert_eq!(skipmap.map, vec![4, 9]);
  }

  #[test]
  fn test_count_skipmap_entries() {
    let mut skipmap = SkipMap::new();
    skipmap.add(4);
    skipmap.add(9);
    assert_eq!(skipmap.count_skipmap_entries(3), 0);
    assert_eq!(skipmap.count_skipmap_entries(4), 1);
    assert_eq!(skipmap.count_skipmap_entries(5), 1);
    assert_eq!(skipmap.count_skipmap_entries(9), 2);
    assert_eq!(skipmap.count_skipmap_entries(10), 2);
  }

  #[test]
  fn test_original_index() {
    let mut skipmap = SkipMap::new();
    skipmap.add(4);
    skipmap.add(9);
    assert_eq!(skipmap.original_index(3), 3); // No skips
    assert_eq!(skipmap.original_index(4), 5); // One skip
    assert_eq!(skipmap.original_index(8), 10); // Two skips
  }
}
