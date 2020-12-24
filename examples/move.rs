#[macro_use]
use ruscape::init;
use ruscape::prelude::*;

fn main() {
    init!(10, 10, 10, Corner, true);
}
