#![feature(plugin)]
#![feature(decl_macro)]
#![feature(type_ascription)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use detexify::Classifier;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

#[get("/")]
fn get_root() -> JsonValue {
    json!({
        "server": "Nöt Betty :(",
        "version": "0.0.2",
    })
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Point {
//     t: i64,
//     x: f64,
//     y: f64,
// }

// type Stroke = Vec<Point>;

#[post("/classify", format = "json", data = "<stroke_sample>")]
fn post_classify(
    stroke_sample: Json<Vec<detexify::Stroke>>,
    classifier: State<Classifier>,
) -> Json<Vec<detexify::Score>> {

    let results = classifier.classify(detexify::StrokeSample::new(stroke_sample.0));
    Json(results)
}

fn main() {
    let classifier = Classifier::default();

    // let snapshot_json: HashMap<String, Vec<Strokes>> = serde_json::from_reader(BufReader::new(file))?;
    // println!("{}", snapshot_json);
    rocket::ignite()
        .mount("/", routes![get_root, post_classify])
        .manage(classifier)
        .launch();
}
