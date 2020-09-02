#![feature(clamp)]

pub mod stokes;
pub use self::stokes::*;
pub mod dtw;
pub mod point;
pub mod classifier;
pub use self::classifier::*;
pub mod stroke_sample;
pub use self::stroke_sample::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
