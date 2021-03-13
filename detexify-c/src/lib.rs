use std::{convert::TryFrom, ffi::CString, os::raw::c_char, ptr};

use detexify::{Classifier, Point, Score, Stroke, StrokeSample, Symbol};

pub struct StrokeBuilder {
    points: Vec<Point>,
}

#[no_mangle]
pub unsafe extern "C" fn stroke_builder_new(capacity: usize) -> *mut StrokeBuilder {
    Box::into_raw(Box::new(StrokeBuilder {
        points: Vec::with_capacity(capacity),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn stroke_builder_add_point(builder: *mut StrokeBuilder, x: f64, y: f64) {
    (*builder).points.push(Point { x, y })
}

/// Frees builder
#[no_mangle]
pub unsafe extern "C" fn stroke_builder_build(builder: *mut StrokeBuilder) -> *mut Stroke {
    let points = Box::from_raw(builder).points;
    Box::into_raw(Box::new(Stroke::new(points)))
}

pub struct StrokeSampleBuilder {
    strokes: Vec<Stroke>,
}

#[no_mangle]
pub unsafe extern "C" fn stroke_sample_new_builder(capacity: usize) -> *mut StrokeSampleBuilder {
    Box::into_raw(Box::new(StrokeSampleBuilder {
        strokes: Vec::with_capacity(capacity),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn stroke_sample_add_stroke(
    builder: *mut StrokeSampleBuilder,
    stroke: *mut Stroke,
) {
    (*builder).strokes.push(*Box::from_raw(stroke));
}

/// Free's builder
#[no_mangle]
pub unsafe extern "C" fn stroke_sample_build(
    builder: *mut StrokeSampleBuilder,
) -> *mut StrokeSample {
    let stroke_sample_builder = Box::from_raw(builder);

    match StrokeSample::new(stroke_sample_builder.strokes) {
        Some(sample) => Box::into_raw(Box::new(sample)),
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn classifier_new_default() -> *mut Classifier {
    Box::into_raw(Box::new(Classifier::default()))
}

pub unsafe extern "C" fn classifier_free(classifier: *mut Classifier) {
    Box::from_raw(classifier);
}

pub struct Scores {
    scores: Vec<Score>,
}

/// Free's sample
#[no_mangle]
pub unsafe extern "C" fn classify(
    classifier: *mut Classifier,
    sample: *mut StrokeSample,
) -> *mut Scores {
    match (*classifier).classify(*Box::from_raw(sample)) {
        Some(scores) => Box::into_raw(Box::new(Scores { scores })),
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn scores_length(scores: *mut Scores) -> usize {
    (*scores).scores.len()
}

#[no_mangle]
pub unsafe extern "C" fn scores_get_command(scores: *mut Scores, i: usize) -> *mut c_char {
    let id = (*scores).scores.get_unchecked(i).id.clone();

    CString::new(Symbol::from_id(&id).unwrap().command)
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn scores_get_package(scores: *mut Scores, i: usize) -> *mut c_char {
    let id = (*scores).scores.get_unchecked(i).id.clone();

    CString::new(Symbol::from_id(&id).unwrap().package)
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn scores_get_font_encoding(scores: *mut Scores, i: usize) -> *mut c_char {
    let id = (*scores).scores.get_unchecked(i).id.clone();

    CString::new(Symbol::from_id(&id).unwrap().font_encoding)
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn scores_get_text_mode(scores: *mut Scores, i: usize) -> bool {
    let id = (*scores).scores.get_unchecked(i).id.clone();
    Symbol::from_id(&id).unwrap().text_mode
}

#[no_mangle]
pub unsafe extern "C" fn scores_get_math_mode(scores: *mut Scores, i: usize) -> bool {
    let id = (*scores).scores.get_unchecked(i).id.clone();
    Symbol::from_id(&id).unwrap().math_mode
}

#[no_mangle]
pub unsafe extern "C" fn scores_free(id: *mut c_char) {
    CString::from_raw(id);
}

#[no_mangle]
pub unsafe extern "C" fn scores_get_score(scores: *mut Scores, i: usize) -> f64 {
    (*scores).scores.get_unchecked(i).score
}
