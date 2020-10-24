fn rotate_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  let mut rotated_image : Vec<Vec<i32>> = vec![vec![0; image.len()]; image.len()];
  for i in 0..image.len() {
    for j in (0..image.len()).rev() {
      rotated_image[i][image.len()-j-1] = image[j][i];
    }
  }
    rotated_image
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rotate_image() {
    // 1
    let mut image : Vec<Vec<i32>> = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    let mut rotated_image : Vec<Vec<i32>> = vec![vec![7,4,1], vec![8,5,2], vec![9,6,3]];
    assert_eq_vectors(rotate_image(image), rotated_image);
    // 2
    image = vec![vec![1]];
    rotated_image = vec![vec![1]];
    assert_eq_vectors(rotate_image(image), rotated_image);
    // 3
    image = vec![vec![10,9,6,3,7], vec![6,10,2,9,7], vec![7,6,3,8,2], vec![8,9,7,9,9], vec![6,8,6,8,2]];
    rotated_image = vec![vec![6,8,7,6,10], vec![8,9,6,10,9], vec![6,7,3,2,6], vec![8,9,8,9,3], vec![2,9,2,7,7]];
    assert_eq_vectors(rotate_image(image), rotated_image);
    // 4
    image = vec![
      vec![40,12,15,37,33,11,45,13,25,3], 
      vec![37,35,15,43,23,12,22,29,46,43], 
      vec![44,19,15,12,30,2,45,7,47,6], 
      vec![48,4,40,10,16,22,18,36,27,48], 
      vec![45,17,36,28,47,46,8,4,17,3], 
      vec![14,9,33,1,6,31,7,38,25,17], 
      vec![31,9,17,11,29,42,38,10,48,6], 
      vec![12,13,42,3,47,24,28,22,3,47], 
      vec![38,23,26,3,23,27,14,40,15,22], 
      vec![8,46,20,21,35,4,36,18,32,3]
    ];
    rotated_image = vec![
      vec![8,38,12,31,14,45,48,44,37,40], 
      vec![46,23,13,9,9,17,4,19,35,12], 
      vec![20,26,42,17,33,36,40,15,15,15], 
      vec![21,3,3,11,1,28,10,12,43,37], 
      vec![35,23,47,29,6,47,16,30,23,33], 
      vec![4,27,24,42,31,46,22,2,12,11], 
      vec![36,14,28,38,7,8,18,45,22,45], 
      vec![18,40,22,10,38,4,36,7,29,13], 
      vec![32,15,3,48,25,17,27,47,46,25], 
      vec![3,22,47,6,17,3,48,6,43,3]
    ];
    assert_eq_vectors(rotate_image(image), rotated_image);
    // 5
    image = vec![
      vec![33,35,8,24,19,1,3,1,4,5], 
      vec![25,27,40,25,17,35,20,3,19,3], 
      vec![9,1,9,30,9,25,32,12,15,22], 
      vec![30,47,25,10,18,1,19,17,43,17], 
      vec![40,46,42,34,18,48,29,40,31,39], 
      vec![37,42,37,19,45,1,4,46,48,13], 
      vec![8,26,31,46,44,24,34,29,12,25], 
      vec![45,48,36,12,33,12,4,45,22,37], 
      vec![33,15,34,25,34,8,50,48,30,28], 
      vec![18,19,22,29,15,43,38,30,8,47]
    ];
    rotated_image = vec![
      vec![18,33,45,8,37,40,30,9,25,33], 
      vec![19,15,48,26,42,46,47,1,27,35], 
      vec![22,34,36,31,37,42,25,9,40,8], 
      vec![29,25,12,46,19,34,10,30,25,24], 
      vec![15,34,33,44,45,18,18,9,17,19], 
      vec![43,8,12,24,1,48,1,25,35,1], 
      vec![38,50,4,34,4,29,19,32,20,3], 
      vec![30,48,45,29,46,40,17,12,3,1], 
      vec![8,30,22,12,48,31,43,15,19,4], 
      vec![47,28,37,25,13,39,17,22,3,5]
    ];
    assert_eq_vectors(rotate_image(image), rotated_image);
  }

  fn assert_eq_vectors(actual_image: Vec<Vec<i32>>, expected_image: Vec<Vec<i32>>) {
    for i in 0..actual_image.len() {
      for j in 0..actual_image.len() {
        assert_eq!(actual_image[i][j], expected_image[i][j]);
      }
    }
  }
}