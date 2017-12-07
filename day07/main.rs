use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;

fn main() {
    let mut args = std::env::args().skip(1);
    let file = args.next().expect("Need file to read");

    let f = File::open(file).expect("Can't open file");
    let reader = BufReader::new(f);

    let mut held_up = HashSet::new();
    let candidates = reader.lines()
        .map(|l| l.unwrap())
        .map(|l| l.split_whitespace().map(|s| s.trim_matches(',').to_string()).collect::<Vec<_>>())
        .filter(|l| l.len() > 3)
        .inspect(|l| {
            for prog in &l[3..] {
                held_up.insert(prog.to_string());
            }
        })
    .map(|l| l[0].to_string())
        .collect::<Vec<_>>();

    for candidate in &candidates {
        if !held_up.contains(candidate) {
            println!("Root: {}", candidate);
            break;
        }
    }
}
