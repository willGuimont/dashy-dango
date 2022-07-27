use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y}
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn norm(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalized(self) -> Self {
        if self.norm() == 0.0{
            self
        }else{
            self / self.norm()
        }
    }

    pub fn scalar_proj(self, rhs:Self) -> f32{
        self.dot(rhs) / rhs.norm()
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}


impl ops::Add<&Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: &Vec2) -> Self::Output {
        self + *rhs
    }
}

impl ops::Add<Vec2> for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        *self + rhs
    }
}

impl ops::Add<f32> for Vec2 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Vec2::new(self.x + rhs, self.y + rhs)
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}


impl ops::Sub<&Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Self::Output {
        self - *rhs
    }
}

impl ops::Sub<Vec2> for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        *self - rhs
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2::new(rhs * self.x, rhs * self.y)
    }
}

impl ops::Mul<&Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: &Vec2) -> Self::Output {
        self * *rhs
    }
}

impl ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1. / rhs)
    }
}
