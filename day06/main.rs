use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

fn redistribute(mut banks: Vec<u32>) -> u32 {
    let mut seen: HashSet<Vec<u32>> = HashSet::new();
    let len = banks.len();
    let mut cycles = 0;

    loop {
        if seen.contains(&banks) {
            break;
        }
        seen.insert(banks.clone());

        let mut max =
            banks.iter()
            .cloned()
            .max()
            .unwrap();
        let mut pos = banks.iter().position(|&c| c == max).unwrap();

        banks[pos] = 0;
        while max > 0 {
            pos = (pos+1) % len;
            banks[pos] += 1;
            max -= 1;
        }

        cycles += 1;
    }

    cycles
}

fn main() {
    let mut args = std::env::args().skip(1);
    let file = args.next().expect("Need file to read");

    let mut f = File::open(file).expect("Can't open file");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("Can't read file");

    let banks = input.split_whitespace()
        .map(|elem| elem.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let n = redistribute(banks);
    println!("Redistribution steps: {}", n);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(5, redistribute(vec![0,2,7,0]));
    }
}
