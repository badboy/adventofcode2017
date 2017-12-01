use std::io::prelude::*;
use std::fs::File;

fn captcha(input: &str) -> u32 {
    let input = input.trim();
    let len = input.len();
    let mut chars = input.chars();
    assert!(len > 1);

    let mut sum = 0;
    let first_val : u32 = chars.next().unwrap().to_digit(10).unwrap();
    let mut last_val = first_val;
    for c in chars {
        let c = c.to_digit(10).unwrap();
        if last_val == c {
            sum += last_val;
        }
        last_val = c;
    }

    if last_val == first_val {
        sum += last_val;
    }

    sum
}

fn wrap_offset(idx: usize, offset: usize, len: usize) -> usize {
    (idx+offset) % len
}

fn captcha2(input: &str) -> u32 {
    let input = input.trim();
    let len = input.len();
    let chars : Vec<_> = input.chars().collect();
    assert!(len > 1);
    let mid = len/2;

    let mut sum = 0;
    for (i,c) in chars.iter().enumerate() {
        let c = c.to_digit(10).unwrap();
        let next = chars[wrap_offset(i, mid, len)].to_digit(10).unwrap();
        if c == next {
            sum += c;
        }
    }

    sum
}

fn main() {
    let mut args = std::env::args().skip(1);
    let file = args.next().expect("Need file to read");

    let mut f = File::open(file).expect("Can't open file");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("Can't read file");

    println!("Part 1: {}", captcha(&input));
    println!("Part 2: {}", captcha2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, captcha("1122"));
    }

    #[test]
    fn case2() {
        assert_eq!(4, captcha("1111"));
    }

    #[test]
    fn case3() {
        assert_eq!(0, captcha("1234"));
    }

    #[test]
    fn case4() {
        assert_eq!(9, captcha("91212129"));
    }

    #[test]
    fn case2_1() {
        assert_eq!(6, captcha2("1212"));
    }

    #[test]
    fn case2_2() {
        assert_eq!(0, captcha2("1221"));
    }

    #[test]
    fn case2_3() {
        assert_eq!(4, captcha2("123425"));
    }

    #[test]
    fn case2_4() {
        assert_eq!(12, captcha2("123123"));
    }

    #[test]
    fn case2_5() {
        assert_eq!(4, captcha2("12131415"));
    }
}
