use crate::sim::Sim;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tuple::*;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Point {
    x: f64,
    y: f64,
}

pub(crate) static ZERO_POINT: Point = Point { x: 0.0, y: 0.0 };
pub(crate) static ONE_POINT: Point = Point { x: 1.0, y: 1.0 };
pub(crate) static HALF_POINT: Point = Point { x: 0.5, y: 0.5 };

pub type Stroke = Vec<Point>;

type Rect = (Point, Point);

fn width(p: Point, q: Point) -> f64 {
    return q.x - p.x;
}

fn height(p: Point, q: Point) -> f64 {
    return q.y - p.y;
}

fn add(p: Point, q: Point) -> Point {
    Point {
        x: p.x + q.x,
        y: p.y + q.y,
    }
}

fn sub(p: Point, q: Point) -> Point {
    Point {
        x: p.x - q.x,
        y: p.y - q.y,
    }
}

fn dot(p: Point, q: Point) -> f64 {
    (p.x * q.x) + (p.y * q.y)
}

fn scalar(p: Point, scalar: f64) -> Point {
    Point {
        x: p.x * scalar,
        y: p.y * scalar,
    }
}

fn norm(p: Point) -> f64 {
    dot(p, p).sqrt()
}

pub(crate) fn euclidean_distance(p: Point, q: Point) -> f64 {
    norm(sub(p, q))
}

pub(crate) fn manhattan_distance(p: Point, q: Point) -> f64 {
    (p.x - q.x).abs() + (p.y - q.y).abs()
}

fn vendomorphism(a11: f64, a12: f64, a21: f64, a22: f64, p: Point) -> Point {
    Point {
        x: a11 * p.x + a12 * p.y,
        y: a21 * p.x + a22 * p.y,
    }
}

fn scale(x: f64, y: f64, p: Point) -> Point {
    vendomorphism(x, 0.0, 0.0, y, p)
}

fn translate(x: f64, y: f64, p: Point) -> Point {
    add(p, Point { x, y })
}

fn slength(stroke: Stroke) -> f64 {
    stroke.windows(2).fold(0.0, |distance, ps| {
        distance + euclidean_distance(ps[0], ps[1])
    })
}

fn bounding_box(mut stroke: Stroke) -> Option<Rect> {
    if stroke.is_empty() {
        return None;
    }

    let point = stroke.pop().unwrap();
    let mut rect: Rect = (point, point);

    for point in stroke {
        if rect.0.x > point.x {
            rect.0.x = point.x;
        }

        if rect.0.y > point.y {
            rect.0.y = point.y;
        }

        if rect.1.x < point.x {
            rect.1.x = point.x;
        }

        if rect.1.y < point.y {
            rect.1.y = point.y;
        }
    }

    Some(rect)
}

fn refit(rect: Rect, stroke: Stroke) -> Stroke {
    if let Some((bb_1, bb_2)) = bounding_box(stroke.clone()) {
        let bb_width = width(bb_1, bb_2);
        let bb_height = height(bb_1, bb_2);
        let target_width = width(rect.0, rect.1);
        let target_height = height(rect.0, rect.1);

        // println!(
        //     "bb_width: {}, bb_height: {}, target_width: {}, target_height: {}",
        //     bb_width, bb_height, target_width, target_height
        // );

        return stroke
            .into_iter()
            .map(|p| {
                let scale_x = if bb_width == 0.0 {
                    1.0
                } else {
                    1.0 / bb_width * target_width
                };

                let scale_y = if bb_height == 0.0 {
                    1.0
                } else {
                    1.0 / bb_height * target_height
                };

                let trans_x = if bb_width == 0.0 {
                    rect.0.x + 0.5 * target_width
                } else {
                    rect.0.x
                };

                let trans_y = if bb_height == 0.0 {
                    rect.0.y + 0.5 * target_height
                } else {
                    rect.0.y
                };

                let trans = Point {
                    x: trans_x,
                    y: trans_y,
                };

                // println!(
                //     "scale_x: {}, scale_y: {}, trans_x: {}, trans_y: {}",
                //     scale_x, scale_y, trans_x, trans_y
                // );

                add(scale(scale_x, scale_y, sub(p, bb_1)), trans)
            })
            .collect();
    }

    stroke
}

pub(crate) fn aspect_fit(source: Rect, target: Rect) -> Rect {
    if source.0 == source.1 {
        let centered = scalar(add(target.0, target.1), 0.5);
        (centered.clone(), centered)
    } else {
        let reset = source.0.clone();
        let source_width = width(source.1, source.0);
        let source_height = height(source.1, source.0);
        let target_width = width(target.1, target.0);
        let target_height = height(target.1, target.0);
        let source_ratio = source_width / source_height;
        let target_ratio = target_width / target_height;

        let scale_factor = if source_ratio > target_ratio {
            1.0 / source_width * target_width
        } else {
            1.0 / source_height * target_height
        };

        let offset = if source_ratio > target_ratio {
            Point {
                x: 0.0,
                y: (target_height - scale_factor * source_height) / 2.0,
            }
        } else {
            Point {
                x: (target_width - scale_factor * source_width) / 2.0,
                y: 0.0,
            }
        };

        source.map(|p| {
            add(
                scale(scale_factor, scale_factor, sub(p, reset)),
                add(offset, target.0),
            )
        })
    }
}

pub(crate) fn aspect_refit(r: Rect, s: Stroke) -> Stroke {
    refit(aspect_fit(bounding_box(s.clone()).unwrap(), r), s)
}

pub(crate) fn unduplicate(s: Stroke) -> Stroke {
    s.into_iter().dedup_by(|p, q| Point::sim(*p, *q)).collect()
}

pub(crate) fn smooth(s: Stroke) -> Stroke {
    if s.len() < 3 {
        return s;
    }

    let first_point = s[0].clone();
    let last_point = s[s.len() - 1].clone();

    let mut smoothed = s
        .into_iter()
        .tuple_windows()
        .map(|(x, y, z)| scalar(add(x, add(y, z)), 1.0 / 3.0))
        .collect::<Stroke>();

    smoothed.insert(0, first_point);
    smoothed.push(last_point);

    smoothed
}

pub(crate) fn redistribute(num: i64, s: Stroke) -> Stroke {
    match num {
        0 => vec![],
        1 => vec![s[0]],
        n => {
            let mut dist = slength(s.clone()) / (n as f64 - 1.0);
            assert!(dist > 0.0);

            let mut new_stroke = Vec::new();
            new_stroke.push(s[0]);

            for (p, q) in s.clone().into_iter().tuple_windows() {
                let dir = sub(q, p);
                let d = norm(dir);

                if d < dist {
                    dist -= d;
                } else {
                    new_stroke.push(add(p, scalar(dir, dist / d)));
                }
            }

            new_stroke.push(s[s.len() - 1]);
            new_stroke
        }
    }
}

fn angle(p: Point, q: Point, r: Point) -> f64 {
    let v = sub(q, p);
    let w = sub(r, q);
    (dot(v, w) / (norm(v) * norm(w))).clamp(-1.0, 1.0).acos()
}

pub(crate) fn dominant(alpha: f64, s: Stroke) -> Stroke {
    let mut new_stroke = Vec::new();
    new_stroke.push(s[0]);

    for (p, q, r) in s.clone().into_iter().tuple_windows() {
        if angle(p, q, r) >= alpha {
            new_stroke.push(q);
        }
    }

    new_stroke.push(s[s.len() - 1]);
    new_stroke
}

#[cfg(test)]
mod tests {

    use super::{bounding_box, refit, Point, HALF_POINT, ONE_POINT, ZERO_POINT};
    use crate::smooth;

    #[test]
    fn test_bounding_box() {
        assert!(bounding_box(vec![]).is_none());

        assert_eq!(
            bounding_box(vec![Point { x: 1.0, y: 1.0 }, Point { x: -1.0, y: -1.0 }]).unwrap(),
            (Point { x: -1.0, y: -1.0 }, Point { x: 1.0, y: 1.0 })
        );
    }

    #[test]
    fn test_refit() {
        assert_eq!(refit((ZERO_POINT, ONE_POINT), vec![]), vec![]);

        assert_eq!(
            refit((ZERO_POINT, ONE_POINT), vec![Point { x: -100.0, y: 0.0 }]),
            vec![HALF_POINT]
        );

        assert_eq!(
            refit(
                (ZERO_POINT, ONE_POINT),
                vec![Point { x: -100.0, y: 0.0 }, Point { x: 0.0, y: 100.0 }]
            ),
            vec![ZERO_POINT, ONE_POINT]
        );
    }

    #[test]
    fn test_smooth() {
        assert_eq!(
            smooth(vec![
                Point {
                    x: 1.2311,
                    y: 1.323
                },
                Point {
                    x: 2.121,
                    y: 2.4123
                },
                Point { x: 3.213, y: 3.251 },
                Point {
                    x: 1.412,
                    y: 4.02441
                }
            ]),
            vec![
                Point {
                    x: 1.2311,
                    y: 1.323
                },
                Point {
                    x: 2.188366666666666,
                    y: 2.3287666666666667
                },
                Point {
                    x: 2.248666666666667,
                    y: 3.229236666666666
                },
                Point {
                    x: 1.412,
                    y: 4.02441
                }
            ]
        )
    }
}
