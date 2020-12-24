use ruscape::prelude::*;

#[test]
fn test_get_set() {
    let w = World::init(10000, 10, 10, Corner, true);
    let patches = w.borrow().patches();
    eprintln!(" = {:#?}", patches.one_of().neighbors());
}
