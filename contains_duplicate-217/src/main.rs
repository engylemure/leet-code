use std::collections::HashSet;

fn main() {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for num in nums {
            if set.contains(&num) {
                return true;
            } else {
                set.insert(num);
            }
        }
        false
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn example_01() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
    }
}
