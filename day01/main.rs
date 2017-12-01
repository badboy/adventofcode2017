use std::io::prelude::*;
use std::fs::File;
use std::str::FromStr;

fn captcha(input: &str) -> u32 {
    let len = input.len();
    assert!(len > 1);
    let mut chars = input.trim().chars();

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

fn main() {
    let mut args = std::env::args().skip(1);
    let file = args.next().expect("Need file to read");

    let mut f = File::open(file).expect("Can't open file");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("Can't read file");

    println!("{}", captcha(&input));
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
}
