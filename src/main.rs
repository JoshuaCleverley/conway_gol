use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    grid_tile_size: f32,
    grid_x: isize,
    grid_y: isize,
    grid: Vec<Vec<Tile>>,
}

type TileVec = Vec<Tile>;
type Grid = Vec<TileVec>;

struct Tile {
    alive: bool,
    nextAlive: bool,
}

impl Tile {
    fn alive() -> Self {
        Tile {
            alive: true,
            nextAlive: false
        }
    }
    fn dead() -> Self {
        Tile {
            alive: false,
            nextAlive: false
        }
    }
}

fn model(app: &App) -> Model {
    let win = app.window_rect();

    let grid_tile_size = 50f32;
    let grid_x = (win.w() / grid_tile_size) as isize;
    let grid_y = (win.h() / grid_tile_size) as isize;

    let mut grid: Grid = Vec::new();

    for _x in 0..grid_x {
        let mut line: TileVec = Vec::new();

        for _y in 0..grid_y {
            line.push(Tile::dead());
        }

        grid.push(line);
    }
    // TESTING START POSITION
    grid[1][1] = Tile::alive();
    grid[1][2] = Tile::alive();
    grid[2][1] = Tile::alive();
    grid[2][2] = Tile::alive();

    grid[5][2] = Tile::alive();
    grid[5][3] = Tile::alive();
    grid[5][4] = Tile::alive();
    //
    
    Model {
        grid_tile_size,
        grid_x,
        grid_y,
        grid
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();

    draw.background().color(BLUE);

    for x in 0..model.grid_x {
        for y in 0..model.grid_y {
            let sq_color = match model.grid[x as usize][y as usize].alive {
                true => BLACK,
                false => WHITE 
            };
            draw.rect()
                .x(-win.w() / 2f32 + model.grid_tile_size / 2f32 + x as f32 * model.grid_tile_size)
                .y( win.h() / 2f32 - model.grid_tile_size / 2f32 - y as f32 * model.grid_tile_size)
                .wh(Vec2::splat(model.grid_tile_size))
                .color(sq_color)
                .stroke(BLACK)
                .stroke_weight(2.0);
        }
    }
    
    draw.to_frame(app, &frame).unwrap();
}