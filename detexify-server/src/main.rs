#![feature(plugin)]
#![feature(decl_macro)]
#![feature(type_ascription)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use detexify::{Classifier, StrokeSample};
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

#[get("/")]
fn get_root() -> JsonValue {
    json!({
        "server": "rust detexify server",
        "version": "0.1.0",
    })
}

#[post("/classify", format = "json", data = "<stroke_sample>")]
fn post_classify(
    stroke_sample: Json<Vec<detexify::Stroke>>,
    classifier: State<Classifier>,
) -> Json<Vec<detexify::Score>> {
    let results = classifier.classify(StrokeSample::new(stroke_sample.0));
    Json(results)
}

fn main() {
    let classifier = Classifier::default();

    rocket::ignite()
        .mount("/", routes![get_root, post_classify])
        .manage(classifier)
        .launch();
}
