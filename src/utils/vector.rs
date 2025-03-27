use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn square_norm(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn normalize(&self) -> Self {
        Self {
            x: self.x / self.square_norm().sqrt(),
            y: self.y / self.square_norm().sqrt(),
            z: self.z / self.square_norm().sqrt(),
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Vector {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Vector {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: other * self.x,
            y: other * self.y,
            z: other * self.z,
        }
    }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VECTOR: Vector = Vector {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    const OTHER_VECTOR: Vector = Vector {
        x: 4.0,
        y: 5.0,
        z: 6.0,
    };
    #[test]
    fn test_add() {
        assert_eq!(
            VECTOR + OTHER_VECTOR,
            Vector {
                x: 5.0,
                y: 7.0,
                z: 9.0
            }
        );
    }

    #[test]
    fn test_add_assign() {
        let mut v = Vector::from(VECTOR);
        v += OTHER_VECTOR;
        assert_eq!(
            v,
            Vector {
                x: 5.0,
                y: 7.0,
                z: 9.0
            }
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            VECTOR - OTHER_VECTOR,
            Vector {
                x: -3.0,
                y: -3.0,
                z: -3.0
            }
        );
    }

    #[test]
    fn test_sub_assign() {
        let mut v = Vector::from(VECTOR);
        v -= OTHER_VECTOR;
        assert_eq!(
            v,
            Vector {
                x: -3.0,
                y: -3.0,
                z: -3.0
            }
        );
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            VECTOR * 2.0,
            Vector {
                x: 2.0,
                y: 4.0,
                z: 6.0
            }
        );
    }

    #[test]
    fn test_mul_assign() {
        let mut v = Vector::from(VECTOR);
        v *= 2.0;
        assert_eq!(
            v,
            Vector {
                x: 2.0,
                y: 4.0,
                z: 6.0
            }
        );
    }

    #[test]
    fn test_dot() {
        assert_eq!(VECTOR.dot(&OTHER_VECTOR), 32f32);
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            VECTOR.cross(&OTHER_VECTOR),
            Vector {
                x: -3.0,
                y: 6.0,
                z: -3.0
            }
        );
    }

    #[test]
    fn test_square_norm() {
        assert_eq!(VECTOR.square_norm(), 14.0);
    }
    #[test]
    fn test_get_normalized() {
        assert_eq!(
            VECTOR.normalize(),
            Vector {
                x: 1.0 / 14f32.sqrt(),
                y: 2.0 / 14f32.sqrt(),
                z: 3.0 / 14f32.sqrt()
            }
        );
    }
}
