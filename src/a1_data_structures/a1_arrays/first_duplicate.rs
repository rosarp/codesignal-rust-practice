use std::collections::HashSet;

fn first_duplicate(a: Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    for v in &a {
        let x = set.insert(v);
        if !x {
            return *v as i32;
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
    let mut a = vec![2, 1, 3, 5, 3, 2];
    assert_eq!(first_duplicate(a), 3);
    // 2
    a = vec![2, 2];
    assert_eq!(first_duplicate(a), 2);
    // 3
    a = vec![2, 4, 3, 5, 1];
    assert_eq!(first_duplicate(a), -1);
    // 4
    a = vec![1];
    assert_eq!(first_duplicate(a), -1);
    // 5
    a = vec![5, 5, 5, 5, 5];
    assert_eq!(first_duplicate(a), 5);
    // 6
    a = vec![2, 1];
    assert_eq!(first_duplicate(a), -1);
    // 7
    a = vec![2, 1, 3];
    assert_eq!(first_duplicate(a), -1);
    // 8
    a = vec![2, 3, 3];
    assert_eq!(first_duplicate(a), 3);
    // 9
    a = vec![3, 3, 3];
    assert_eq!(first_duplicate(a), 3);
    // 10
    a = vec![8, 4, 6, 2, 6, 4, 7, 9, 5, 8];
    assert_eq!(first_duplicate(a), 6);
    // 11
    a = vec![10, 6, 8, 4, 9, 1, 7, 2, 5, 3];
    assert_eq!(first_duplicate(a), -1);
    // 12
    a = vec![1, 1, 2, 2, 1];
    assert_eq!(first_duplicate(a), 1);
  }
}