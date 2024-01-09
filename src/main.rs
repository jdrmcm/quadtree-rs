mod quadtree;

use rand::random;
use quadtree::*;

fn main() {
    let boundary = Rectangle::new(200, 200, 200, 200);
    let mut qt = QuadTree::new(boundary, 4);
    let dim = 400.0;

    for _ in 0..10000 {
        let p = Point::new(random::<f64>() * dim, random::<f64>() as f64 * dim);
        qt.insert(p);
    }

    qt.show();
}