use std::io::prelude::*;
use std::fs::File;

fn captcha(input: &str) -> u32 {
    let input = input.trim();
    let len = input.len();
    assert!(len > 1);
    let chars = input
        .chars()
        .cycle()
        .map(|c| c.to_digit(10).unwrap())
        .take(len + 1)
        .collect::<Vec<_>>();

    chars.windows(2).fold(0, |acc, vals| if vals[0] == vals[1] {
        acc + vals[0]
    } else {
        acc
    })
}

fn wrap_offset(idx: usize, offset: usize, len: usize) -> usize {
    (idx+offset) % len
}

fn captcha2(input: &str) -> u32 {
    let input = input.trim();
    let len = input.len();
    let chars : Vec<_> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    assert!(len > 1);
    let mid = len/2;

    let mut sum = 0;
    for (i,&c) in chars.iter().enumerate() {
        let next = chars[wrap_offset(i, mid, len)];
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
