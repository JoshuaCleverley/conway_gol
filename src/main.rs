use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();

    let grid_tile_size = 24f32;
    let grid_x = (win.w() / grid_tile_size) as isize;
    let grid_y = (win.h() / grid_tile_size) as isize;

    for x in 0..grid_x {
        for y in 0..grid_y {
            draw.rect()
                .x( -win.w() / 2f32 + grid_tile_size / 2f32 + x as f32 * grid_tile_size)
                .y(  win.h() / 2f32 - grid_tile_size / 2f32 - y as f32 * grid_tile_size)
                .wh(Vec2::splat(grid_tile_size))
                .color(WHITE)
                .stroke(BLACK)
                .stroke_weight(2.0);
        }
    }
    
    draw.background().color(BLUE);
    draw.to_frame(app, &frame).unwrap();
}