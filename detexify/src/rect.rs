use crate::point::Point;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Rect {
    pub(crate) lower_left: Point,
    pub(crate) upper_right: Point,
}

impl Rect {
    pub(crate) fn new(lower_left: Point, upper_right: Point) -> Self {
        Rect {
            lower_left,
            upper_right,
        }
    }

    pub(crate) fn from_point(p: Point) -> Self {
        Rect {
            lower_left: p,
            upper_right: p,
        }
    }

    pub(crate) fn is_point(self) -> bool {
        self.lower_left == self.upper_right
    }

    pub(crate) fn width(self) -> f64 {
        self.upper_right.x - self.lower_left.x
    }

    pub(crate) fn height(self) -> f64 {
        self.upper_right.y - self.lower_left.y
    }

    pub(crate) fn map_points<F: FnMut(Point) -> Point>(self, mut f: F) -> Rect {
        Rect {
            lower_left: f(self.lower_left),
            upper_right: f(self.upper_right),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Rect;
    use crate::point::ZERO_POINT;

    #[test]
    fn test_rect_point() {
        assert!(Rect::from_point(ZERO_POINT).is_point());
    }

    #[test]
    fn test_width_height() {
        assert_eq!(Rect::from_point(ZERO_POINT).width(), 0.0);
        assert_eq!(Rect::from_point(ZERO_POINT).height(), 0.0);
    }

}
