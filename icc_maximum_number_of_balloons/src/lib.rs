struct Solution;
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut chars_count = vec![0; "balon".len()];

        for l in text.chars() {
            match l {
                'b' => chars_count[0] += 1,
                'a' => chars_count[1] += 1,
                'l' => chars_count[2] += 1,
                'o' => chars_count[3] += 1,
                'n' => chars_count[4] += 1,
                _ => {},
            }
        }

        chars_count[2] /= 2;
        chars_count[3] /= 2;

        let mut ans = chars_count[0];
        for c in chars_count {
            ans = i32::min(ans, c);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = Solution::max_number_of_balloons("nlaebolko".to_owned());
        assert_eq!(result, 1);
    }

    #[test]
    fn t2() {
        let result = Solution::max_number_of_balloons("loonbalxballpoon".to_owned());
        assert_eq!(result, 2);
    }

    #[test]
    fn t3() {
        let result = Solution::max_number_of_balloons("leetcode".to_owned());
        assert_eq!(result, 0);
    }

    #[test]
    fn t4() {
        let result = Solution::max_number_of_balloons("krhizmmgmcrecekgyljqkldocicziihtgpqwbticmvuyznragqoyrukzopfmjhjjxemsxmrsxuqmnkrzhgvtgdgtykhcglurvppvcwhrhrjoislonvvglhdciilduvuiebmffaagxerjeewmtcwmhmtwlxtvlbocczlrppmpjbpnifqtlninyzjtmazxdbzwxthpvrfulvrspycqcghuopjirzoeuqhetnbrcdakilzmklxwudxxhwilasbjjhhfgghogqoofsufysmcqeilaivtmfziumjloewbkjvaahsaaggteppqyuoylgpbdwqubaalfwcqrjeycjbbpifjbpigjdnnswocusuprydgrtxuaojeriigwumlovafxnpibjopjfqzrwemoinmptxddgcszmfprdrichjeqcvikynzigleaajcysusqasqadjemgnyvmzmbcfrttrzonwafrnedglhpudovigwvpimttiketopkvqw".to_owned());
        assert_eq!(result, 10);
    }

}
