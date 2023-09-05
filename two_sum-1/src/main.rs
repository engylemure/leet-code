use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut numbers: HashMap<i32, usize> = HashMap::new();
        for (idx, num) in nums.into_iter().enumerate() {
            match numbers.get(&(target - num)) {
                Some(pos) => return vec![*pos as i32, idx as i32],
                None => {
                    numbers.insert(num, idx);
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn example_01() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
