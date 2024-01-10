use nannou::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;
use rand::random;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rectangle {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, w:f64, h: f64) -> Self {
        Self {
            x,
            y,
            w,
            h,
        }
    }

    fn contains(&self, point: Point) -> bool {
        return point.x > (self.x - self.w) &&
                point.x < (self.x + self.w) &&
                point.y > (self.y - self.h) &&
                point.y < (self.y + self.h)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuadTree {
    boundary: Rectangle,
    capacity: usize,
    points: Vec<Point>,
    northwest: Option<Box<Self>>,
    northeast: Option<Box<Self>>,
    southwest: Option<Box<Self>>,
    southeast: Option<Box<Self>>,
    divided: bool,
}

impl QuadTree {
    pub fn new(boundary: Rectangle, capacity: usize) -> Self {
        Self {
            boundary,
            capacity,
            points: Vec::new(),
            northwest: None,
            northeast: None,
            southwest: None,
            southeast: None,
            divided: false,
        }
    }

    fn subdivide(&mut self) {
        let b = &self.boundary;

        let ne = Rectangle::new(b.x + b.w/2.0, b.y - b.h / 2.0, b.w/2.0, b.h/2.0);
        self.northeast = Some(Box::new(QuadTree::new(ne, self.capacity)));
        let nw = Rectangle::new(b.x - b.w/2.0, b.y - b.h / 2.0, b.w/2.0, b.h/2.0);
        self.northwest = Some(Box::new(QuadTree::new(nw, self.capacity)));
        let se = Rectangle::new(b.x + b.w/2.0, b.y + b.h / 2.0, b.w/2.0, b.h/2.0);
        self.southeast = Some(Box::new(QuadTree::new(se, self.capacity)));
        let sw = Rectangle::new(b.x - b.w/2.0, b.y + b.h / 2.0, b.w/2.0, b.h/2.0);
        self.southwest = Some(Box::new(QuadTree::new(sw, self.capacity)));

        self.divided = true;
    }

    pub fn insert(&mut self, point: Point) {

        if !self.boundary.contains(point) {
            return;
        }

        if self.points.len() < self.capacity {
            self.points.push(point);
        } else {
            if !self.divided {
                self.subdivide();
            }
            self.northeast.as_mut().unwrap().insert(point);
            self.northwest.as_mut().unwrap().insert(point);
            self.southeast.as_mut().unwrap().insert(point);
            self.southwest.as_mut().unwrap().insert(point);
        }
    }

    pub fn show(&self) {
        let serialized = serde_json::to_string(self).unwrap();
        write_data(&serialized);
        nannou::sketch(view).run();
    }
}

fn write_data(data: &String) {
    fs::write("data.json", data).expect("failed to write");
}

fn read_data() -> QuadTree {
    let data = fs::read_to_string("data.json").expect("failed to read");
    let deserialized: QuadTree = serde_json::from_str(&*data).unwrap();
    deserialized
}

fn view(app: &App, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(DARKOLIVEGREEN);

    show_quadtree(Box::new(read_data()),  &draw);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

fn show_quadtree(qt: Box<QuadTree>, draw: &Draw) {
    draw.rect()
        .x_y(qt.boundary.x as f32, qt.boundary.y as f32)
        .w(qt.boundary.w as f32*2.0)
        .h(qt.boundary.h as f32*2.0)
        .hsv(random(), random(), random());

    if qt.divided {
        show_quadtree(qt.northeast.unwrap(), draw);
        show_quadtree(qt.northwest.unwrap(), draw);
        show_quadtree(qt.southeast.unwrap(), draw);
        show_quadtree(qt.southwest.unwrap(), draw);
    }

    for point in qt.points {
        draw.ellipse()
            .color(WHITE)
            .x_y(point.x as f32, point.y as f32)
            .w(5.0)
            .h(5.0);
    }
}