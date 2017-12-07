use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut args = std::env::args().skip(1);
    let file = args.next().expect("Need file to read");

    let f = File::open(file).expect("Can't open file");
    let reader = BufReader::new(f);

    let mut held_up = HashSet::new();
    let nodes = reader.lines()
        .map(|l| l.unwrap())
        .map(|l| l.split_whitespace().map(|s| s.trim_matches(',').to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let weights : HashMap<String, u32> = nodes.iter()
        .map(|node| {
            (node[0].to_string(), node[1].trim_matches(|c| c == '(' || c == ')').parse().unwrap())
        })
        .collect();
    let tree : HashMap<String, Vec<String>> = nodes.iter()
        .map(|node| {
            let children = if node.len() > 3 {
                node[3..].to_vec()
            } else {
                vec![]
            };
            (node[0].to_string(), children)
        })
        .collect();

    let (internals, _) : (Vec<_>, Vec<_>)= nodes.iter().partition(|l| l.len() > 3);

    let candidates = internals
        .into_iter()
        .inspect(|l| {
            for prog in &l[3..] {
                held_up.insert(prog.to_string());
            }
        })
        .map(|l| l[0].to_string())
        .collect::<Vec<_>>();

    let root : String = candidates.into_iter().find(|candidate| !held_up.contains(candidate)).unwrap();
    println!("Root: {}", root);

    println!();
    dfs(&tree, &weights, &root, 0);
}

fn dfs(tree: &HashMap<String, Vec<String>>, weights: &HashMap<String, u32>, root: &str, level: usize) -> u32 {
    let weight = weights[root];
    let children = match tree.get(root) {
        Some(c) => c,
        None => panic!("Invalid node"),
    };

    let subtree_sums = children.iter().map(|c| {
        dfs(tree, weights, c, level+1)
    })
    .collect::<Vec<_>>();

    let total_sum = weight + subtree_sums.iter().sum::<u32>();

    let valid = if subtree_sums.len() == 0 {
        true
    } else {
        let f = subtree_sums[0];
        subtree_sums.iter().all(|&s| s == f)
    };

    if !valid {
        println!("Found invalid in subtree of {}, children:", root);
        for (i, c) in children.iter().enumerate() {
            println!("{}\t{}", c, subtree_sums[i]);
        }
        let f = subtree_sums[0];
        let adj = subtree_sums.iter().map(|&c| (c as i32) - (f as i32)).max().unwrap();
        println!("Needs adjustment by {}", adj);
        println!();
    }

    total_sum
}
