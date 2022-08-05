use crate::math_utils::Vec2;

#[derive(Copy, Clone)]
pub struct Point(Vec2);

impl Point {
    pub fn new(x: f32, y: f32) -> Point { Point { 0: Vec2 { x, y } } }

    pub fn from_vector(vec: Vec2) -> Point {
        Point { 0: vec }
    }

    pub fn to_vector(self, other: Self) -> Vec2 { other.0 - self.0 }
}

pub struct Quadrilateral {
    points: [Point; 4],
}

impl Quadrilateral {
    pub fn new(points: [Point; 4]) -> Self { Quadrilateral { points } }

    pub fn rect_inter(&self, other: &Quadrilateral) -> bool {
        self.verify_projection(&other) || other.verify_projection(&self)
    }

    fn verify_projection(&self, other: &Quadrilateral) -> bool {
        for i in 0..2 {
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
        true
    }
}

pub fn create_box(pos: Vec2, width: f32, height: f32) -> Quadrilateral {
    let x = pos.x;
    let y = pos.y;
    let p1 = Point::new(x, y);
    let p2 = Point::new(x, y + height);
    let p3 = Point::new(x + width, y + height);
    let p4 = Point::new(x + width, y);
    Quadrilateral::new([p1, p2, p3, p4])
}
