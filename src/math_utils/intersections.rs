use std::cmp::{max, min};
use std::collections::LinkedList;
use std::ops::Deref;

use crate::math_utils::Vec2;
use crate::trace;

// TODO rect-rect intersection, and other various intersection algorithms
#[derive(Copy, Clone)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self { Point { x, y } }

    pub fn to_vector(self, other: Self) -> Vec2 { Vec2::new(other.x - self.x, other.y - self.y) }
}

pub struct Quadrilateral {
    points: [Point; 4],
}

impl Quadrilateral {
    pub fn new(points: [Point; 4]) -> Self { Quadrilateral { points } }

    pub fn rect_inter(self, other: Quadrilateral) -> bool {
        self.verify_projection(&other) || other.verify_projection(&self)
    }

    fn verify_projection(&self, other: &Quadrilateral) -> bool {
        for i in 0..1 {
            let slice = &other.points[i..i + 2];
            let vec_b = slice[0].to_vector(slice[1]);
            let mut smallest = f32::INFINITY;
            let mut greatest = f32::NEG_INFINITY;
            for point in self.points.iter().enumerate() {
                let vec_a = slice[0].to_vector(*point.1);
                let projection = vec_a.scalar_proj(vec_b);
                smallest = smallest.min(projection);
                greatest = greatest.max(projection);
            }
            if greatest < 0.0 || smallest > vec_b.norm()
            {
                return false;
            }
        }
        return true;
    }
}