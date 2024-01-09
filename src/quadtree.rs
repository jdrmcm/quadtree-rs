use nannou::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;

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
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Rectangle {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self {
            x,
            y,
            w,
            h,
        }
    }

    fn contains(&self, point: Point) -> bool {
        return point.x > (self.x - self.w) as f64 &&
                point.x < (self.x + self.w) as f64 &&
                point.y > (self.y - self.h) as f64 &&
                point.y < (self.y + self.h) as f64
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

        let ne = Rectangle::new(b.x + b.w/2, b.y - b.h / 2, b.w/2, b.h/2);
        self.northeast = Some(Box::new(QuadTree::new(ne, self.capacity)));
        let nw = Rectangle::new(b.x - b.w/2, b.y - b.h / 2, b.w/2, b.h/2);
        self.northwest = Some(Box::new(QuadTree::new(nw, self.capacity)));
        let se = Rectangle::new(b.x + b.w/2, b.y + b.h / 2, b.w/2, b.h/2);
        self.southeast = Some(Box::new(QuadTree::new(se, self.capacity)));
        let sw = Rectangle::new(b.x - b.w/2, b.y + b.h / 2, b.w/2, b.h/2);
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

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
