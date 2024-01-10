mod quadtree;

use rand::random;
use quadtree::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let boundary = Rectangle::new(200.0, 200.0, 200.0, 200.0);
    let mut qt = QuadTree::new(boundary, 4);
    let dim = 400.0;

    for _ in 0..args[1].parse().expect("ERR: Invalid Number") {
        let p = Point::new(random::<f64>() * dim, random::<f64>() * dim);
        qt.insert(p);
    }

    qt.show();
}