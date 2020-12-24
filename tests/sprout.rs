use ruscape::prelude::*;

#[test]
fn sprout_test() {
    let w = World::init(10000, 10, 10, Corner, true);

    let mut turtles = w.borrow().turtles();
    turtles.own(vec!["some"]);
    turtles.ask(|t| {
        t.setxy(t.random_xcor(), t.random_ycor());
        t.patch_here().sprout(1);
    });

    eprintln!(
        " = {:#?}",
        w.borrow().turtles().max_one_of(|t| t.who()).who()
    );
}
