use std::env;
use Direction::*;

fn offset_to_coords(offset: usize, width: usize) -> (usize, usize) {
    let y = offset / width;
    let x = offset - y * width;
    (x, y)
}

fn coords_to_offset((x, y): (usize, usize), width: usize) -> usize {
    x * width + y
}

fn display_spiral(spiral: &[usize]) {
    let root = (spiral.len() as f32).sqrt() as usize;
    for y in 0..root {
        for x in 0..root {
            print!("{:>4} ", spiral[coords_to_offset((x,y),root)]);
        }
        println!();
    }
}

#[derive(Clone,Copy,Debug)]
enum Direction {
    Right(usize),
    Left(usize),
    Up(usize),
    Down(usize),
}

struct Spiral {
    data: Vec<usize>,
    width: usize,
}

impl Spiral {
    fn new(max: usize) -> Spiral {
        let root = ((max as f32).sqrt()) as usize + 1;
        let root = if root%2 == 0 {
            (root + 1) as usize
        } else {
            root as usize
        };
        let realmax = root*root;
        let mut spiral = vec![0; realmax as usize];
        let middle = realmax / 2;

        spiral[middle as usize] = 1;

        let mut pos = offset_to_coords(middle, root);
        let mut layer = 1;
        let mut movement = Right(1);

        for i in 0..realmax {
            let val = i+1;
            spiral[coords_to_offset(pos, root)] = val;

            let (x, y) = pos;

            movement = match movement {
                Right(1) => {
                    pos = (x+1, y);
                    Up(layer)
                }
                Up(1) => {
                    pos = (x, y-1);
                    layer += 1;
                    Left(layer)
                }
                Left(1) => {
                    pos = (x-1, y);
                    Down(layer)
                }
                Down(1) => {
                    pos = (x, y+1);
                    layer += 1;
                    Right(layer)
                }

                Right(left) => {
                    pos = (x+1, y);
                    Right(left-1)
                }
                Left(left) => {
                    pos = (x-1, y);
                    Left(left-1)
                }
                Up(left) => {
                    pos = (x, y-1);
                    Up(left-1)
                }
                Down(left) => {
                    pos = (x, y+1);
                    Down(left-1)
                }
            }
        }

        if root < 10 {
            display_spiral(&spiral);
        }

        Spiral {
            data: spiral,
            width: root,
        }
    }


    fn find(&self, n: usize) -> (usize, usize) {
        let pos = self.data.iter().position(|&val| val == n).unwrap_or(0);
        offset_to_coords(pos, self.width)
    }

    fn distance_to_center(&self, (x,y): (usize, usize)) -> usize {
        let middle = self.data.len() / 2;
        let midpos = offset_to_coords(middle, self.width);

        ((midpos.0 as isize - x as isize).abs() + (midpos.1 as isize - y as isize).abs()) as usize
    }
}
fn main() {
    let n = env::args().skip(1).next().map(|s| s.parse::<usize>().unwrap()).unwrap_or(25);
    let spiral = Spiral::new(n);
    let pos = spiral.find(n);
    println!("Position for {}: {:?}", n, pos);
    println!("Distance to center: {}", spiral.distance_to_center(pos));
}
