


#![warn(float_cmp_const)]
#![allow(unused, no_effect, unnecessary_operation)]

const ONE: f32 = 1.0;
const TWO: f32 = 2.0;

fn eq_one(x: f32) -> bool {
    if x.is_nan() { false } else { x == ONE } // no error, inside "eq" fn
}

fn main() {
    // has errors
    1f32 == ONE;
    TWO == ONE;
    TWO != ONE;
    ONE + ONE == TWO;
    1 as f32 == ONE;

    let v = 0.9;
    v == ONE;
    v != ONE;

    // no errors, lower than or greater than comparisons
    v < ONE;
    v > ONE;
    v <= ONE;
    v >= ONE;

    // no errors, zero and infinity values
    ONE != 0f32;
    TWO == 0f32;
    ONE != ::std::f32::INFINITY;
    ONE == ::std::f32::NEG_INFINITY;

    // Note: float_cmp will warn as expected on cases where there are no float constants
    //       e.g. v == 1.0
}
