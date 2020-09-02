use crate::stokes::{euclidean_distance, Point};

pub(crate) trait Sim<T> {
    fn sim(a: T, b: T) -> bool;
}

trait Simord<T>
where
    T: Sim<T> + PartialOrd,
{
    fn lt_sim(a: T, b: T) -> bool {
        a < b || T::sim(a, b)
    }

    fn gt_sim(a: T, b: T) -> bool {
        a > b || T::sim(a, b)
    }
}

static DELTA: f64 = 1e-10;

impl Sim<f64> for f64 {
    fn sim(a: f64, b: f64) -> bool {
        (a - b).abs() < DELTA
    }
}

impl Simord<f64> for f64 {}

impl Sim<Point> for Point {
    fn sim(a: Point, b: Point) -> bool {
        euclidean_distance(a, b) < DELTA
    }
}

impl<T> Sim<Vec<T>> for Vec<T>
where
    T: Sim<T>,
{
    fn sim(a: Vec<T>, b: Vec<T>) -> bool {
        a.into_iter().zip(b).map(|(p, q)| T::sim(p, q)).all(|b| b)
    }
}

#[cfg(test)]
mod tests {
    use super::{Sim, DELTA};

    static SMALL_DELTA: f64 = 1e-11;

    #[test]
    fn sim_f64() {
        assert!(f64::sim(1.0, 1.0));
        assert!(f64::sim(1.0, 1.0 + SMALL_DELTA));
        assert!(f64::sim(1.0, 1.0 - SMALL_DELTA));

        assert!(!f64::sim(1.0, 2.0));
        assert!(!f64::sim(1.0, 1.0 + DELTA));
        assert!(!f64::sim(1.0, 1.0 - DELTA));
    }

    #[test]
    fn sim_vec() {
        assert!(Vec::sim(
            vec![1.0, 3.0, 5.0],
            vec![1.0 + SMALL_DELTA, 3.0 - SMALL_DELTA, 5.0]
        ));
    }
}
