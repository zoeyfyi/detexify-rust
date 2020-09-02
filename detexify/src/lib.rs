#![feature(clamp)]

pub mod stokes;
pub use self::stokes::*;
pub mod sim;
pub use self::sim::*;
pub mod dtw;
pub use self::dtw::*;
pub mod classifier;
pub use self::classifier::*;
pub mod stroke_sample;
pub use self::stroke_sample::*;
pub mod json;
pub use self::json::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
