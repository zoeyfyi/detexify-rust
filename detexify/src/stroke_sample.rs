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
    pub fn new(mut strokes: Vec<Stroke>) -> Self {
        for stroke in strokes.iter_mut() {
            stroke.dedup();
            stroke.smooth();
            stroke.aspect_refit(Rect::new(ZERO_POINT, ONE_POINT));
            stroke.redistribute(10);
            stroke.dedup();
            stroke.dominant(2.0 * PI * 15.0 / 360.0)
        }

        strokes.truncate(10);

        StrokeSample { strokes }

        // StrokeSample {
        //     strokes: strokes
        //         .into_iter()
        //         .map(unduplicate)
        //         .map(smooth)
        //         .map(|s| aspect_refit(Rect::new(ZERO_POINT, ONE_POINT), s))
        //         .map(|s| redistribute(10, s))
        //         .map(unduplicate)
        //         .map(|s| dominant(2.0 * PI * 15.0 / 360.0, s))
        //         .take(10)
        //         .collect(),
        // }
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
