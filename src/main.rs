use macroquad::prelude::*;

const BOARD_WIDTH: u16 = 32;
const BOARD_HEIGHT: u16 = 32;
const BOARD_SIZE: usize = BOARD_WIDTH as usize * BOARD_HEIGHT as usize;
const CELL_SIZE: f32 = 12.0;
const INITIAL_LAYER_WIDTH: u16 = 12;

enum Direction {
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct Game {
    board: [u16; BOARD_SIZE],
    is_over: bool,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [0; BOARD_SIZE],
            is_over: false,
        }
    }

    fn set_cell(&mut self, x: u16, y: u16, val: u16) {
        self.board[(x + (y * BOARD_WIDTH)) as usize] = val;
    }

    fn get_cell(&self, x: u16, y: u16) -> u16 {
        self.board[(x + (y * BOARD_WIDTH)) as usize]
    }

    fn get_layer_width(&self, level: u16) -> u16 {
        let mut count = 0;
        for x in 0..BOARD_WIDTH {
            count += self.get_cell(x, level);
        }
        count
    }
}

#[macroquad::main("Stackem")]
async fn main() {
    let mut game = Game::new();
    let mut move_direction = Direction::RIGHT;
    let mut last_update = get_time();
    let mut speed = 0.1;
    let mut level = 0;

    for i in 0..INITIAL_LAYER_WIDTH {
        game.set_cell(i, level, 1);
    }

    loop {
        clear_background(BLACK);

        if game.is_over && is_key_pressed(KeyCode::Space) {
            game.is_over = false;
            speed = 0.1;
            level = 0;

            for y in 0..BOARD_HEIGHT {
                for x in 0..BOARD_WIDTH {
                    game.set_cell(x, y, 0);
                }
            }

            for i in 0..INITIAL_LAYER_WIDTH {
                game.set_cell(i, level, 1);
            }
        }

        if !game.is_over && is_key_pressed(KeyCode::Down) {
            if level > 0 {
                for i in 0..BOARD_WIDTH {
                    if game.get_cell(i, level - 1) == 0 {
                        game.set_cell(i, level, 0);
                    }
                }
            }

            let layer_width = game.get_layer_width(level);

            if layer_width > 0 {
                speed *= 0.97;
                level += 1;

                if level < BOARD_HEIGHT {
                    for i in 0..layer_width {
                        game.set_cell(i, level, 1);
                    }
                } else {
                    game.is_over = true;
                }
            } else {
                game.is_over = true;
            }
        }

        if !game.is_over && get_time() - last_update > speed {
            last_update = get_time();

            match move_direction {
                Direction::LEFT => {
                    if game.get_cell(0, level) == 1 {
                        move_direction = Direction::RIGHT
                    }
                }
                Direction::RIGHT => {
                    if game.get_cell(BOARD_WIDTH - 1, level) == 1 {
                        move_direction = Direction::LEFT
                    }
                }
            }

            match move_direction {
                Direction::LEFT => {
                    let mut i = 0;
                    loop {
                        if game.get_cell(i, level) == 1 {
                            game.set_cell(i, level, 0);
                            game.set_cell(i - 1, level, 1);
                        }
                        if i == BOARD_WIDTH - 1 {
                            break;
                        }
                        i += 1;
                    }
                }
                Direction::RIGHT => {
                    let mut i = BOARD_WIDTH - 1;
                    loop {
                        if game.get_cell(i, level) == 1 {
                            game.set_cell(i, level, 0);
                            game.set_cell(i + 1, level, 1);
                        }
                        if i == 0 {
                            break;
                        }
                        i -= 1;
                    }
                }
            }
        }

        if game.is_over {
            if level == BOARD_HEIGHT {
                draw_text(format!("YOU ARE WINNER!").as_str(), 8.0, 60.0, 60.0, GREEN);
            } else {
                draw_text(format!("GAME OVER").as_str(), 8.0, 60.0, 60.0, GREEN);
            }
        }

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if game.get_cell(x, y) == 1 {
                    let rx = x as f32 * (CELL_SIZE + 2.0) + 8.0;
                    let ry = (BOARD_HEIGHT as f32 - y as f32) * (CELL_SIZE + 2.0) + 80.0;
                    draw_rectangle(rx, ry, CELL_SIZE, CELL_SIZE, GREEN);
                }
            }
        }

        draw_text(format!("LEVEL: {}", level).as_str(), 8.0, 20.0, 20.0, GREEN);
        next_frame().await
    }
}
