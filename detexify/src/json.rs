use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct JsonPoint {
    x: f64,
    y: f64,
}
