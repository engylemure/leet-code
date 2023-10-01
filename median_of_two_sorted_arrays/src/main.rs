fn main() {
    println!("Hello, world!");
}

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let nums1_len = nums1.len();
        let nums2_len = nums2.len();
        let mut iter1 = nums1.into_iter();
        let mut iter2 = nums2.into_iter();
        let mut num1 = iter1.next();
        let mut num2 = iter2.next();
        let sorted_array: Vec<_> = (0..(nums1_len + nums2_len) / 2 + 1)
            .map(|_| match (num1, num2) {
                (None, Some(val2)) => {
                    num2 = iter2.next();
                    val2
                }
                (Some(val1), None) => {
                    num1 = iter1.next();
                    val1
                }
                (Some(val1), Some(val2)) => {
                    if val2 < val1 {
                        num2 = iter2.next();
                        val2
                    } else {
                        num1 = iter1.next();
                        val1
                    }
                }
                (None, None) => unreachable!(),
            })
            .collect();
        let sorted_array_len = sorted_array.len();
        if (nums1_len + nums2_len) % 2 == 0 {
            (sorted_array[sorted_array_len - 2] + sorted_array[sorted_array_len - 1]) as f64 / 2.0
        } else {
            sorted_array[sorted_array_len - 1] as f64
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_01() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
    }

    #[test]
    fn example_02() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
