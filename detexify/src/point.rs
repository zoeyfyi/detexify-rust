use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul, Sub};

pub(crate) const ZERO_POINT: Point = Point { x: 0.0, y: 0.0 };
pub(crate) const ONE_POINT: Point = Point { x: 1.0, y: 1.0 };

const DELTA: f64 = 1e-10;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

fn clamp(v: f64, min: f64, max: f64) -> f64 {
    if v < min {
        return min;
    } else if v > max {
        return max;
    } else {
        return v;
    }
}

impl Point {
    pub(crate) fn dot(p: Point, q: Point) -> f64 {
        (p.x * q.x) + (p.y * q.y)
    }

    pub(crate) fn norm(self) -> f64 {
        Point::dot(self, self).sqrt()
    }

    pub(crate) fn euclidean_distance(p: Point, q: Point) -> f64 {
        (p - q).norm()
    }

    pub(crate) fn manhattan_distance(p: Point, q: Point) -> f64 {
        (p.x - q.x).abs() + (p.y - q.y).abs()
    }

    pub(crate) fn scale_x(self, x: f64) -> Point {
        Point {
            x: self.x * x,
            y: self.y,
        }
    }

    pub(crate) fn scale_y(self, y: f64) -> Point {
        Point {
            x: self.x,
            y: self.y * y,
        }
    }

    pub(crate) fn approx_eq(p: Point, q: Point) -> bool {
        Point::euclidean_distance(p, q) < DELTA
    }

    pub(crate) fn angle(p: Point, q: Point, r: Point) -> f64 {
        let v = q - p;
        let w = r - q;
        clamp(Point::dot(v, w) / (v.norm() * w.norm()), -1.0, 1.0).acos()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SMALL_DELTA: f64 = 1e-11;

    #[test]
    fn test_add_points() {
        assert_eq!(
            Point { x: 1.0, y: 0.0 } + Point { x: 2.0, y: 3.0 },
            Point { x: 3.0, y: 3.0 }
        );
    }

    #[test]
    fn test_sub_points() {
        assert_eq!(
            Point { x: 1.0, y: 0.0 } - Point { x: 2.0, y: 3.0 },
            Point { x: -1.0, y: -3.0 }
        );
    }

    #[test]
    fn test_mul_point() {
        assert_eq!(Point { x: 1.0, y: 3.0 } * 4.0, Point { x: 4.0, y: 12.0 })
    }

    #[test]
    fn test_approx_eq_vec() {
        assert!(Point::approx_eq(
            Point { x: 1.0, y: 3.0 },
            Point {
                x: 1.0 + SMALL_DELTA,
                y: 3.0 - SMALL_DELTA
            }
        ));
    }
}
