fn main() {
    println!("Hello, world!");
}

pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut s_letters: [u32; 256] = [0; 256];
        let mut t_letters: [u32; 256] = [0; 256];
        let mut char_s_buf: [u8; 1] = [0; 1];
        let mut char_t_buf: [u8; 1] = [0; 1];
        for (char_s, char_t) in s.chars().zip(t.chars()) {
            char_s.encode_utf8(&mut char_s_buf);
            char_t.encode_utf8(&mut char_t_buf);
            s_letters[char_s_buf[0] as usize] += 1;
            t_letters[char_t_buf[0] as usize] += 1;
        }
        dbg!(&s_letters, &t_letters);
        s_letters == t_letters
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn example_01() {
        assert_eq!(
            Solution::is_anagram("anagram".into(), "nagaram".into()),
            true
        );
    }

    #[test]
    fn example_02() {
        assert_eq!(Solution::is_anagram("car".into(), "rat".into()), false)
    }

    #[test]
    fn example_03() {
        let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string();
        let t = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbba".to_string();
        assert_eq!(Solution::is_anagram(s, t), false);
    }
}
