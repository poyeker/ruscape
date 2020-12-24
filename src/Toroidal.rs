use std::f64;

pub(crate) fn toroidal_distance(val1: f64, val2: f64, dim: f64) -> f64 {
    let abs = (val1 - val2).abs();
    if abs <= dim / 2.0 {
        abs
    } else {
        dim - abs
    }
}

pub(crate) fn toroidal_transform(val: f64, min: f64, max: f64) -> f64 {
    if val >= min && val <= max {
        val
    } else {
        let val = (val - min) % (max - min);
        if val < 0. {
            val + max
        } else {
            val + min
        }
    }
}

#[test]
fn test() {
    eprintln!(
        "toroidal_transform(11., -10., 10.) = {:#?}",
        toroidal_transform(29., -0.5, 10.5)
    );
}
