use crate::classifier::Sample;
use crate::dtw::gdtw;
use crate::{
    smooth,
    stokes::{
        aspect_refit, dominant, manhattan_distance, redistribute, unduplicate, Stroke, ONE_POINT,
        ZERO_POINT,
    },
};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StrokeSample {
    strokes: Vec<Stroke>,
}

impl StrokeSample {
    pub fn new(strokes: Vec<Stroke>) -> Self {
        StrokeSample {
            strokes: strokes
                .into_iter()
                .map(unduplicate)
                .map(smooth)
                .map(|s| aspect_refit((ZERO_POINT, ONE_POINT), s))
                .map(|s| redistribute(10, s))
                .map(unduplicate)
                .map(|s| dominant(2.0 * PI * 15.0 / 360.0, s))
                .take(10)
                .collect(),
        }
    }
}

impl Sample<StrokeSample> for StrokeSample {
    fn distance(a: StrokeSample, b: StrokeSample) -> f64 {
        gdtw(manhattan_distance, a.strokes.concat(), b.strokes.concat())
    }
}
