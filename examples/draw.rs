use macroquad::*;
use ruscape::prelude::*;

const MAX_PXCOR: i64 = 100;
const MAX_PYCOR: i64 = 100;
const PATCH_SIZE: i32 = 10;
fn window_conf() -> Conf {
    Conf {
        window_title: "Window".to_owned(),
        fullscreen: false,
        window_width: PATCH_SIZE * (MAX_PXCOR as i32 + 1),
        window_height: PATCH_SIZE * (MAX_PYCOR as i32 + 1),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let w = World::init(100, MAX_PXCOR, MAX_PYCOR, Corner, true);
    let patches = w.borrow().patches();
    let turtles = w.borrow().turtles();
    //patches.ask(|p| p.set_random_pcolor());
    turtles.ask(|t| {
        t.set_random_color();
    });
    turtles.ask(|t| {
        t.random_headings().fd(t.rng().f64() * 6.);
    });
    turtles.ask(|t| {
        t.random_headings().fd(t.rng().f64() * 6.);
    });
    loop {
        clear_background(BLACK);
        let sq_height = screen_height() / (MAX_PYCOR + 1) as f32;
        let sq_width = screen_width() / (MAX_PXCOR + 1) as f32;
        patches.values().for_each(|p| {
            draw_rectangle(
                (p.pxcor() * PATCH_SIZE as i64) as f32,
                (p.pycor() * PATCH_SIZE as i64) as f32,
                sq_width,
                sq_height,
                p.pcolor(),
            )
        });
        turtles.values().for_each(|t| {
            draw_circle(
                (t.xcor() * PATCH_SIZE as f64) as f32,
                (t.ycor() * PATCH_SIZE as f64) as f32,
                10.,
                t.color(),
            )
        });

        next_frame().await
    }
}
