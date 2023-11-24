use crate::Solution;

impl Solution {
  pub fn convert(s: String, num_rows: i32) -> String {
      let mut add = 0;
      let mut flag = false;
      let mut string_array: Vec<String> = vec![String::new(); num_rows as usize];

      for c in s.chars() {
          string_array[add] = string_array[add].clone() + &c.to_string();

          if add == num_rows as usize - 1 {
              flag = true;
          }
          else if add == 0 {
              flag = false;
          }
          if flag {
              add = add - 1; 
          } 
          else {
              add = add + 1;
          }
      }

      let mut result = String::new();
      for i in 0..num_rows {
          result.push_str(&string_array[i as usize]);
      }
      result
  }
}