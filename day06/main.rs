use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::collections::hash_map::Entry::*;

fn redistribute(mut banks: Vec<u32>) -> (u32, u32) {
    let mut seen: HashMap<Vec<u32>, u32> = HashMap::new();
    let len = banks.len();
    let mut cycles = 0;
    let first_seen;

    loop {
        match seen.entry(banks.clone()) {
            Occupied(e) => {
                first_seen = *e.get();
                break;
            }
            Vacant(e) => {
                e.insert(cycles);
            }
        }

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

    (cycles-first_seen, cycles)
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

    let (many, n) = redistribute(banks);
    println!("Redistribution steps: {}", n);
    println!("How many? {}", many);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!((4,5), redistribute(vec![0,2,7,0]));
    }
}
