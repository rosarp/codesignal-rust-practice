use std::collections::HashSet;

#[allow(dead_code)]
fn first_duplicate(input_vec: Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    for val in &input_vec {
        let is_inserted = set.insert(val);
        if !is_inserted {
            return *val as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_first_duplicate() {
    // 1
    let mut input_vec = vec![2, 1, 3, 5, 3, 2];
    assert_eq!(first_duplicate(input_vec), 3);
    // 2
    input_vec = vec![2, 2];
    assert_eq!(first_duplicate(input_vec), 2);
    // 3
    input_vec = vec![2, 4, 3, 5, 1];
    assert_eq!(first_duplicate(input_vec), -1);
    // 4
    input_vec = vec![1];
    assert_eq!(first_duplicate(input_vec), -1);
    // 5
    input_vec = vec![5, 5, 5, 5, 5];
    assert_eq!(first_duplicate(input_vec), 5);
    // 6
    input_vec = vec![2, 1];
    assert_eq!(first_duplicate(input_vec), -1);
    // 7
    input_vec = vec![2, 1, 3];
    assert_eq!(first_duplicate(input_vec), -1);
    // 8
    input_vec = vec![2, 3, 3];
    assert_eq!(first_duplicate(input_vec), 3);
    // 9
    input_vec = vec![3, 3, 3];
    assert_eq!(first_duplicate(input_vec), 3);
    // 10
    input_vec = vec![8, 4, 6, 2, 6, 4, 7, 9, 5, 8];
    assert_eq!(first_duplicate(input_vec), 6);
    // 11
    input_vec = vec![10, 6, 8, 4, 9, 1, 7, 2, 5, 3];
    assert_eq!(first_duplicate(input_vec), -1);
    // 12
    input_vec = vec![1, 1, 2, 2, 1];
    assert_eq!(first_duplicate(input_vec), 1);
  }
}