use core::f64;

use rand::Rng;
use crate::constants::{MAX_COORD,NUM_LINES};

#[derive(Clone, Debug, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
} 

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        Point::new(rng.random_range(0..=MAX_COORD), rng.random_range(0..=MAX_COORD))
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        Line::new(Point::random(rng), Point::random(rng))
    }
}

#[derive(Clone, Debug)]
pub struct Genotype {
  pub lines: Vec<Line>,
}

impl Genotype {
  pub fn new_random(rng: &mut impl Rng) -> Self {
    let lines = (0..NUM_LINES).map(|_| Line::random(rng)).collect();
    Genotype { lines }
  }
}

#[derive(Debug,Clone)]
pub struct Individual {
    pub genotype: Genotype,
    pub fitness: f64,
}

impl Individual {
    pub fn new(genotype: Genotype) -> Self {
        Individual { genotype, fitness: f64::MAX }
    }
}