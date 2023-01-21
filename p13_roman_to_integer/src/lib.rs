use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    println!("running test for {}", &s);
    let mut romans = HashMap::new();
    romans.insert(String::from("I"), 1);
    romans.insert(String::from("V"), 5);
    romans.insert(String::from("X"), 10);
    romans.insert(String::from("L"), 50);
    romans.insert(String::from("C"), 100);
    romans.insert(String::from("D"), 500);
    romans.insert(String::from("M"), 1000);

    let size: usize = s.len();
    match size {
        1..=15 => {},
        _ => panic!("1 <= s.length <= 15"),
    }
    let mut num: i32;
    let mut last: i32 = 0;
    let mut bucket: i32 = 0;
    let mut sum: i32 = 0;


    for (i, c) in s.chars().enumerate() {
        let char = c.to_string();
        num = romans.get(&char).copied().unwrap();
        println!("char: {char}, index: {i}, num: {num}");
        if i == 0 {
            bucket = num;
        } else if last == num {
            bucket = num + bucket;
        } else if last > num {
            sum = sum + bucket;
            bucket = num;
        } else if last < num {
            bucket = num - bucket;
            sum = sum + bucket;
            bucket = 0;
        }
        last = num;

        if i+1 == size {
            sum = sum + bucket;
            println!("last round. sum = {sum}");
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let result = roman_to_int(String::from("III"));
        assert_eq!(result, 3);
    }

    #[test]
    fn case_special() {
        let result = roman_to_int(String::from("XVI"));
        assert_eq!(result, 16);
    }

    #[test]
    fn case_special_2() {
        let result = roman_to_int(String::from("IIIV"));
        assert_eq!(result, 2);
    }

    #[test]
    fn case_2() {
        let result = roman_to_int(String::from("LVIII"));
        assert_eq!(result, 58);
    }

    #[test]
    fn case_3() {
        let result = roman_to_int(String::from("MCMXCIV"));
        assert_eq!(result, 1994);
    }

}
