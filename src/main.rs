mod quadtree;

use rand::random;
use quadtree::*;
use std::env;

fn main() {
    let dim = 400.0;
    let args: Vec<String> = env::args().collect();
    let boundary = Rectangle::new(dim / 2.0, dim / 2.0, dim / 2.0, dim / 2.0);
    let mut qt = QuadTree::new(boundary, 4);

    for _ in 0..args[1].parse().expect("ERR: Invalid Number") {
        let p = Point::new(random::<f64>() * dim, random::<f64>() * dim);
        qt.insert(p);
    }

    qt.show();
}