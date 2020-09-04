use crate::classifier::Sample;
use crate::dtw::gdtw;
use crate::{
    point::{Point, ONE_POINT, ZERO_POINT},
    rect::Rect,
    stroke::Stroke,
};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StrokeSample {
    strokes: Vec<Stroke>,
}

impl StrokeSample {
    pub fn new(mut strokes: Vec<Stroke>) -> Option<Self> {
        strokes = strokes.into_iter().filter(|s| !s.is_empty()).collect();

        if strokes.is_empty() {
            return None;
        }

        strokes.truncate(10);
        for stroke in strokes.iter_mut() {
            stroke.dedup();
            stroke.smooth();
            stroke.aspect_refit(Rect::new(ZERO_POINT, ONE_POINT));
            stroke.redistribute(10);
            stroke.dedup();
            stroke.dominant(2.0 * PI * 15.0 / 360.0)
        }

        Some(StrokeSample { strokes })
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.strokes.is_empty()
    }
}

impl Sample<StrokeSample> for StrokeSample {
    fn distance(a: StrokeSample, b: StrokeSample) -> f64 {
        gdtw(
            Point::manhattan_distance,
            Stroke::concat(a.strokes),
            Stroke::concat(b.strokes),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{point::Point, Stroke, StrokeSample};

    #[test]
    fn test_sample() {
        let points = vec![
            Point { x: 166.0, y: 80.0 },
            Point { x: 156.0, y: 104.0 },
            Point { x: 82.0, y: 182.0 },
            Point { x: 48.0, y: 194.0 },
            Point { x: 28.0, y: 127.0 },
            Point { x: 39.0, y: 115.0 },
            Point { x: 59.0, y: 106.0 },
            Point { x: 120.0, y: 106.0 },
            Point { x: 135.0, y: 115.0 },
            Point { x: 149.0, y: 129.0 },
            Point { x: 160.0, y: 145.0 },
            Point { x: 207.0, y: 200.0 },
        ];

        let expected_points = vec![
            Point {
                x: 0.7569169960474308,
                y: 0.1442687747035573,
            },
            Point {
                x: 0.40477099163285507,
                y: 0.5540160467382995,
            },
            Point {
                x: 0.1626658042136413,
                y: 0.6500724952223573,
            },
            Point {
                x: 4.308955299830499e-3,
                y: 0.4971489109610397,
            },
            Point {
                x: 0.14894547060976254,
                y: 0.3286712800588731,
            },
            Point {
                x: 0.41775887969016,
                y: 0.32246108567927273,
            },
            Point {
                x: 0.651671871693856,
                y: 0.44047560390735446,
            },
            Point {
                x: 0.9999999999999999,
                y: 0.8557312252964425,
            },
        ];

        let sample = StrokeSample::new(vec![Stroke::new(points)]).unwrap();
        assert_eq!(sample.strokes.len(), 1);

        for (i, &point) in sample.strokes[0].clone().points().enumerate() {
            assert!(Point::approx_eq(point, expected_points[i]));
        }
    }

    #[test]
    fn test_bad_samples() {
        let strokes = Vec::new();
        assert!(StrokeSample::new(strokes.clone()).is_none());

        let strokes = vec![Stroke::new(vec![])];
        assert!(StrokeSample::new(strokes.clone()).is_none());

        let strokes = vec![Stroke::new(vec![Point { x: 0.5, y: 0.5 }])];
        assert_eq!(StrokeSample::new(strokes.clone()).unwrap().strokes, strokes);
    }
}
