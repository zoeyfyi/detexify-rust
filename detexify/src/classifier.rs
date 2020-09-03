use crate::stroke_sample::StrokeSample;
use itertools::Itertools;
use serde::Serialize;
use std::{collections::HashMap};

pub(crate) trait Sample<T> {
    fn distance(a: T, b: T) -> f64;
}

#[derive(Debug)]
struct Hit<T> {
    sample_score: f64,
    sample: T,
}

#[derive(Debug, Serialize)]
pub struct Score {
    pub id: String,
    pub score: f64,
}

pub struct Classifier {
    samples: HashMap<String, Vec<StrokeSample>>,
}

impl Classifier {
    pub fn new() -> Classifier {
        Classifier {
            samples: HashMap::new(),
        }
    }

    pub fn from_snapshot<R: std::io::Read>(reader: R) -> serde_json::Result<Classifier> {
        let samples = serde_json::from_reader(reader);
        samples.map(|s| Classifier { samples: s })
    }

    pub fn classify(&self, unknown: StrokeSample) -> Vec<Score> {
        self
            .samples
            .iter()
            .map(|(id, samples)| {
                let mean_dist = samples
                    .iter()
                    .cloned()
                    .map(|s| StrokeSample::distance(unknown.clone(), s))
                    .sorted_by(|x, y| x.partial_cmp(y).unwrap())
                    .take(2)
                    .fold(0.0, |acc, x| acc + x)
                    / 2.0;

                (id, mean_dist)
            })
            .map(|(id, dist)| Score {
                id: id.clone(),
                score: dist,
            })
            .sorted_by(|x, y| x.score.partial_cmp(&y.score).unwrap())
            .collect()
    }
}

impl Default for Classifier {
    fn default() -> Self {
        Classifier {
            samples: serde_json::from_slice(include_bytes!("../snapshot.json")).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Classifier;

    #[test]
    fn default_classifier() {
        Classifier::default();
    }
}

// fn insert_with_limit<T: Sample>(
//     limit: i64,
//     identifier: String,
//     sample: T,
//     map: HashMap<String, Vec>,
// ) -> HashMap<String, Vec> {
//     todo!()
// }

// fn train_classifier<T: Sample>(classifier: Classifier, identifier: String, sample: T) {
//     insert_with_limit(
//         classifier.sample_limit,
//         identifier,
//         sample,
//         classifier.samples,
//     );
// }
