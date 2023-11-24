mod add_two_number;
mod zig_zag;
mod reverse_int;

pub struct Solution;

fn main() {
    let scores = [10, 5, 20, 20, 4, 5, 2, 25, 1];
    let result = breakingRecords(&scores);
    println!("Result: {:?}", result);
}


fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    let mut max = 0;
    let mut min = 0;
    let mut max_count = 0;
    let mut min_count = 0;
    for i in 0..scores.len() {
        if i == 0 {
            max = scores[i];
            min = scores[i];
        } else {
            if scores[i] > max {
                max = scores[i];
                max_count = max_count + 1;
            }
            if scores[i] < min {
                min = scores[i];
                min_count = min_count + 1;
            }
        }
    }
    return vec![max_count, min_count];
}