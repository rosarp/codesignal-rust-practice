use std::collections::HashMap;

fn is_crypt_solution(crypt: Vec<String>, solution: Vec<Vec<char>>) -> bool {
  let mut map : HashMap<char, u8> = HashMap::new();
  for v in &solution {
    map.insert(v[0], v[1] as u8 - 48);
  }
  if (crypt[0].len() > 1 && *map.get(&crypt[0].chars().nth(0).unwrap()).unwrap() == 0u8)
      || (crypt[1].len() > 1 && *map.get(&crypt[1].chars().nth(0).unwrap()).unwrap() == 0u8)
      || (crypt[2].len() > 1 && *map.get(&crypt[2].chars().nth(0).unwrap()).unwrap() == 0u8) {
    return false;
  }
  let mut carry = 0u8;
  let mut iter0 = crypt[0].chars().rev();
  let mut iter1 = crypt[1].chars().rev();
  let mut iter2 = crypt[2].chars().rev();
  for _ in 0..crypt[2].len() {
    let v0 = match_char(iter0.next(), &map);
    let v1 = match_char(iter1.next(), &map);
    let v2 = match_char(iter2.next(), &map);
    println!("{} {} {}", v0, v1, v2);
    if v0 + v1 > 9 {
      if (v0 + v1) % 10 != v2 - carry {
        println!("-- {} {}", (v0 + v1) % 10, v2);
        return false;
      }
      carry = 1u8;
    } else if v0 + v1 + carry != v2 {
      println!("++ {} {}", v0 + v1 + carry, v2);
      return false;
    } else {
      carry = 0u8;
    }
  }
  if carry != 0u8 {
    false
  } else {
    true
  }
}

fn match_char(ch: Option<char>, map: &HashMap<char, u8>) -> u8 {
  match ch {
    Some(x) => return *map.get(&x).unwrap(),
    None => return 0u8
  };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_crypt_solution() {
    // 1
    let mut crypt: Vec<String> = vec![String::from("SEND"), String::from("MORE"), String::from("MONEY")];
    let mut solution: Vec<Vec<char>> = vec![ vec!['O','0'], vec!['M','1'], vec!['Y','2'], 
      vec!['E','5'], vec!['N','6'], vec!['D','7'], vec!['R','8'], vec!['S','9']];
    assert_eq!(is_crypt_solution(crypt, solution), true);
    // 2
    crypt = vec![String::from("TEN"), String::from("TWO"), String::from("ONE")];
    solution = vec![vec!['O','1'], vec!['T','0'], vec!['W','9'], vec!['E','5'], vec!['N','4']];
    assert_eq!(is_crypt_solution(crypt, solution), false);
    // 3
    crypt = vec![String::from("ONE"), String::from("ONE"), String::from("TWO")];
    solution = vec![vec!['O','2'], vec!['T','4'], vec!['W','6'], vec!['E','1'], vec!['N','3']];
    assert_eq!(is_crypt_solution(crypt, solution), true);
    // 4
    crypt = vec![String::from("ONE"), String::from("ONE"), String::from("TWO")];
    solution = vec![vec!['O','0'], vec!['T','1'], vec!['W','2'], vec!['E','5'], vec!['N','6']];
    assert_eq!(is_crypt_solution(crypt, solution), false);
    // 5
    crypt = vec![String::from("A"), String::from("A"), String::from("A")];
    solution = vec![vec!['A','0']];
    assert_eq!(is_crypt_solution(crypt, solution), true);
    // 6
    crypt = vec![String::from("A"), String::from("B"), String::from("C")];
    solution = vec![vec!['A','5'], vec!['B','6'], vec!['C','1']];
    assert_eq!(is_crypt_solution(crypt, solution), false);
    // 7
    crypt = vec![String::from("AA"), String::from("AA"), String::from("AA")];
    solution = vec![vec!['A','0']];
    assert_eq!(is_crypt_solution(crypt, solution), false);
    // 8
    crypt = vec![String::from("A"), String::from("A"), String::from("A")];
    solution = vec![vec!['A','1']];
    assert_eq!(is_crypt_solution(crypt, solution), false);
    // 9
    crypt = vec![String::from("AA"), String::from("AA"), String::from("BB")];
    solution = vec![vec!['A','1'], vec!['B','2']];
    assert_eq!(is_crypt_solution(crypt, solution), true);
    // 10
    crypt = vec![String::from("BAA"), String::from("CAB"), String::from("DAB")];
    solution = vec![vec!['A','0'], vec!['B','1'], vec!['C','2'], vec!['D','4']];
    assert_eq!(is_crypt_solution(crypt, solution), false);
  }
}
