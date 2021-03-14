use std::{convert::TryFrom, ffi::CString, os::raw::c_char, ptr};

use detexify::{iter_symbols, Classifier, Point, Score, Stroke, StrokeSample, Symbol};

pub struct StrokeBuilder {
    points: Vec<Point>,
}

/// Creates a new stroke builder
#[no_mangle]
pub unsafe extern "C" fn stroke_builder_new(capacity: usize) -> *mut StrokeBuilder {
    Box::into_raw(Box::new(StrokeBuilder {
        points: Vec::with_capacity(capacity),
    }))
}

/// Adds a point to the stroke
#[no_mangle]
pub unsafe extern "C" fn stroke_builder_add_point(builder: *mut StrokeBuilder, x: f64, y: f64) {
    (*builder).points.push(Point { x, y })
}

/// Returns the stroke and frees `builder`
#[no_mangle]
pub unsafe extern "C" fn stroke_builder_build(builder: *mut StrokeBuilder) -> *mut Stroke {
    let points = Box::from_raw(builder).points;
    Box::into_raw(Box::new(Stroke::new(points)))
}

pub struct StrokeSampleBuilder {
    strokes: Vec<Stroke>,
}

/// Creates a new stroke sample builder
#[no_mangle]
pub unsafe extern "C" fn stroke_sample_builder_new(capacity: usize) -> *mut StrokeSampleBuilder {
    Box::into_raw(Box::new(StrokeSampleBuilder {
        strokes: Vec::with_capacity(capacity),
    }))
}

/// Adds a stroke to the stroke sample and frees the stroke
#[no_mangle]
pub unsafe extern "C" fn stroke_sample_builder_add_stroke(
    builder: *mut StrokeSampleBuilder,
    stroke: *mut Stroke,
) {
    (*builder).strokes.push(*Box::from_raw(stroke));
}

/// Returns a stroke sample and free's `builder`
#[no_mangle]
pub unsafe extern "C" fn stroke_sample_builder_build(
    builder: *mut StrokeSampleBuilder,
) -> *mut StrokeSample {
    let stroke_sample_builder = Box::from_raw(builder);

    match StrokeSample::new(stroke_sample_builder.strokes) {
        Some(sample) => Box::into_raw(Box::new(sample)),
        None => ptr::null_mut(),
    }
}

/// Returns the default classifier
#[no_mangle]
pub unsafe extern "C" fn classifier_new_default() -> *mut Classifier {
    Box::into_raw(Box::new(Classifier::default()))
}

/// Free's a classifier
pub unsafe extern "C" fn classifier_free(classifier: *mut Classifier) {
    Box::from_raw(classifier);
}

pub struct Scores {
    scores: Vec<Score>,
}

/// Classifiy the sample returning scores and free's `sample`
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

/// Returns the length of the list of symbols
#[no_mangle]
pub unsafe extern "C" fn scores_length(scores: *mut Scores) -> usize {
    (*scores).scores.len()
}

/// Returns the `i`-th score of `scores`
#[no_mangle]
pub unsafe extern "C" fn scores_get_score(scores: *mut Scores, i: usize) -> f64 {
    (*scores).scores.get_unchecked(i).score
}

/// Returns the `i`-th symbol of `scores`, callers responsible for calling `symbol_free` once finished
#[no_mangle]
pub unsafe extern "C" fn scores_get_symbol(scores: *mut Scores, i: usize) -> *const Symbol {
    match Symbol::from_id(&(*scores).scores[i].id) {
        Some(symbol) => Box::into_raw(Box::new(symbol)),
        None => ptr::null_mut(),
    }
}

/// Free's scores
#[no_mangle]
pub unsafe extern "C" fn scores_free(scores: *mut Scores) {
    Box::from_raw(scores);
}

/// Gets the command of the `i`-th score
#[no_mangle]
pub unsafe extern "C" fn symbol_get_command(
    symbol: *const Symbol,
    buffer: *mut c_char,
    len: usize,
) {
    let s = CString::new((*symbol).command).unwrap();
    ptr::copy(
        s.into_raw(),
        buffer,
        usize::min(len, (*symbol).command.len()),
    );
}

/// Gets the package of the `i`-th score
#[no_mangle]
pub unsafe extern "C" fn symbol_get_package(
    symbol: *const Symbol,
    buffer: *mut c_char,
    len: usize,
) {
    let s = CString::new((*symbol).package).unwrap();
    ptr::copy(
        s.into_raw(),
        buffer,
        usize::min(len, (*symbol).package.len()),
    );
}

/// Gets the font encoding of the `i`-th score
#[no_mangle]
pub unsafe extern "C" fn symbol_get_font_encoding(
    symbol: *const Symbol,
    buffer: *mut c_char,
    len: usize,
) {
    let s = CString::new((*symbol).font_encoding).unwrap();
    ptr::copy(
        s.into_raw(),
        buffer,
        usize::min(len, (*symbol).font_encoding.len()),
    );
}

/// Gets the text mode of the `i`-th score
#[no_mangle]
pub unsafe extern "C" fn symbol_get_text_mode(symbol: *const Symbol) -> bool {
    (*symbol).text_mode
}

/// Gets the math mode of the `i`-th score
#[no_mangle]
pub unsafe extern "C" fn symbol_get_math_mode(symbol: *const Symbol) -> bool {
    (*symbol).math_mode
}

/// Frees `symbol`
#[no_mangle]
pub unsafe extern "C" fn symbol_free(symbol: *mut Symbol) {
    Box::from_raw(symbol);
}

/// Returns the total number of symbols
#[no_mangle]
pub unsafe extern "C" fn symbols_count() -> usize {
    iter_symbols().count()
}

/// Returns the `i`-th symbol
#[no_mangle]
pub unsafe extern "C" fn symbols_get(i: usize) -> *mut Symbol {
    match iter_symbols().nth(i) {
        Some(symbol) => Box::into_raw(Box::new(symbol)),
        None => ptr::null_mut(),
    }
}
