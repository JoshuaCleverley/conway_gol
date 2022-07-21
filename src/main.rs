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

    let grid_tile_size = 10f32;
    let grid_x = (win.w() / grid_tile_size) as isize;
    let grid_y = (win.h() / grid_tile_size) as isize;

    let mut grid: Grid = Vec::new();

    for _x in 0..grid_x {
        let mut line: TileVec = Vec::new();
        for _y in 0..grid_y {
            if random_f32() < 0.5f32 {
                line.push(Tile::dead());
            } else {
                line.push(Tile::alive());
            }
            
        }
        grid.push(line);
    }

    Model {
        grid_tile_size,
        grid_x,
        grid_y,
        grid
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    

    for x in 0..model.grid_x {
        for y in 0..model.grid_y {
            // Count neighbours
            let mut neighbours = 0;

            for xoff in -1..=1 {
                for yoff in -1..=1 {
                    if xoff == 0 && yoff == 0 { continue; }
                    let xt = xoff + x;
                    let yt = yoff + y;
                    if xt >= 0 && xt < model.grid_x && yt >= 0 && yt < model.grid_y {
                        if model.grid[xt as usize][yt as usize].alive == true {
                            neighbours = neighbours + 1;
                        }
                    }
                }
            }

            // Set next state
            if neighbours == 3 {
                model.grid[x as usize][y as usize].nextAlive = true;
            } else if neighbours == 2 && model.grid[x as usize][y as usize].alive == true {
                model.grid[x as usize][y as usize].nextAlive = true;
            } else {
                model.grid[x as usize][y as usize].nextAlive = false;
            }
        }
    }

    for x in 0..model.grid_x {
        for y in 0..model.grid_y {
            model.grid[x as usize][y as usize].alive = model.grid[x as usize][y as usize].nextAlive;
        }
    }
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
                .stroke_weight(1.0);
        }
    }
    
    draw.to_frame(app, &frame).unwrap();
}