use crate::Solution;

impl Solution {
  pub fn reverse(x: i32) -> i32 {
    let mut string;
    let result: i32;
      if x > 0 {
          string = x.to_string();
          string = string.chars().rev().collect();
          result = string.parse::<i32>().unwrap_or(0);
          return result;
      } else {
          string = x.to_string();
          // Remove the negative sign
          string.remove(0);
          string = string.chars().rev().collect();
          result = string.parse::<i32>().unwrap_or(0);
          return result * -1;
      }
  }
}