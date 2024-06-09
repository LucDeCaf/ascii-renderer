use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone)]
pub struct Vector2<T>(pub T, pub T);

impl Vector2<f32> {
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1
    }
}

impl Add<Vector2<f32>> for Vector2<f32> {
    type Output = Vector2<f32>;

    fn add(self, rhs: Vector2<f32>) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Vector2<f32>> for Vector2<f32> {
    type Output = Vector2<f32>;

    fn sub(self, rhs: Vector2<f32>) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<f32> for Vector2<f32> {
    type Output = Vector2<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<f32> for Vector2<f32> {
    type Output = Vector2<f32>;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl AddAssign<Vector2<f32>> for Vector2<f32> {
    fn add_assign(&mut self, rhs: Vector2<f32>) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl SubAssign<Vector2<f32>> for Vector2<f32> {
    fn sub_assign(&mut self, rhs: Vector2<f32>) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl MulAssign<Vector2<f32>> for Vector2<f32> {
    fn mul_assign(&mut self, rhs: Vector2<f32>) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
    }
}

impl DivAssign<Vector2<f32>> for Vector2<f32> {
    fn div_assign(&mut self, rhs: Vector2<f32>) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
    }
}
