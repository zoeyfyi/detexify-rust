#![feature(clamp)]

pub mod point;
pub mod rect;
pub mod stroke;
pub use self::stroke::*;
pub use self::classifier::*;
pub mod classifier;
pub mod dtw;
pub mod stroke_sample;
pub use self::stroke_sample::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
