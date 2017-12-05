use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn jumps_to_outside(mut jumps: Vec<i32>) -> u32 {
    let mut steps = 0;
    let len = jumps.len();
    let mut pos = 0i32;

    while pos >= 0 && pos < len as i32 {
        let nex = jumps[pos as usize];
        jumps[pos as usize] += 1;

        pos += nex;
        steps += 1;
    }

    steps
}

fn main() {
    let mut args = std::env::args().skip(1);
    let file = args.next().expect("Need file to read");

    let f = File::open(file).expect("Can't open file");
    let reader = BufReader::new(f);

    let jumps = reader.lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap()).
        collect::<Vec<_>>();

    let n = jumps_to_outside(jumps);
    println!("Jumps: {}", n);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(5, jumps_to_outside(vec![0,3,0,1,-3]));
    }
}
