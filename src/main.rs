use macroquad::prelude::*;

const BOARD_WIDTH: usize = 32;
const BOARD_HEIGHT: usize = 32;
const CELL_SIZE: f32 = 12.0;
const INITIAL_LAYER_WIDTH: u8 = 12;

enum Direction {
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct Game {
    board: [[u8; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Game {
    fn new() -> Game {
        Game {
            board: [[0; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }
}

fn get_layer_width(board: [[u8; BOARD_WIDTH]; BOARD_HEIGHT], level: u8) -> u8 {
    let mut count: u8 = 0;
    for el in board[level as usize].iter() {
        count += el;
    }
    count
}

#[macroquad::main("Stackem")]
async fn main() {
    let mut game = Game::new();
    let mut move_direction = Direction::RIGHT;
    let mut last_update = get_time();
    let mut speed = 0.1;
    let mut level = 0;
    let mut game_over = false;
    for i in 0..INITIAL_LAYER_WIDTH {
        game.board[level as usize][i as usize] = 1;
    }

    loop {
        clear_background(BLACK);

        if game_over && is_key_pressed(KeyCode::Space) {
            game_over = false;
            speed = 0.1;
            level = 0;
            for i in 0..BOARD_HEIGHT {
                for j in 0..BOARD_WIDTH {
                    game.board[i][j] = 0;
                }
            }
            for i in 0..INITIAL_LAYER_WIDTH {
                game.board[level as usize][i as usize] = 1;
            }
        }

        if !game_over && is_key_pressed(KeyCode::Down) {
            if level > 0 {
                for i in 0..BOARD_WIDTH - 1 {
                    if game.board[(level - 1) as usize][i as usize] == 0 {
                        game.board[level as usize][i as usize] = 0;
                    }
                }
            }
            let layer_width = get_layer_width(game.board, level);
            if layer_width > 0 {
                speed *= 0.97;
                level += 1;
                if (level as usize) < BOARD_HEIGHT {
                    for i in 0..layer_width {
                        game.board[level as usize][i as usize] = 1;
                    }
                } else {
                    game_over = true;
                }
            } else {
                game_over = true;
            }
        }

        if game_over {
            if (level as usize) == BOARD_HEIGHT {
                draw_text(format!("YOU ARE WINNER!").as_str(), 8.0, 60.0, 60.0, GREEN);
            } else {
                draw_text(format!("GAME OVER").as_str(), 8.0, 60.0, 60.0, GREEN);
            }
        } else if get_time() - last_update > speed {
            last_update = get_time();
            match move_direction {
                Direction::LEFT => {
                    let first = game.board[level as usize][0];
                    if first == 1 {
                        move_direction = Direction::RIGHT
                    }
                }
                Direction::RIGHT => {
                    let last = game.board[level as usize][BOARD_WIDTH - 1];
                    if last == 1 {
                        move_direction = Direction::LEFT
                    }
                }
            }
            match move_direction {
                Direction::LEFT => {
                    let mut i = 0;
                    loop {
                        if game.board[level as usize][i as usize] == 1 {
                            game.board[level as usize][i as usize] = 0;
                            game.board[level as usize][i - 1 as usize] = 1;
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
                        if game.board[level as usize][i as usize] == 1 {
                            game.board[level as usize][i as usize] = 0;
                            game.board[level as usize][(i + 1) as usize] = 1;
                        }
                        if i == 0 {
                            break;
                        }
                        i -= 1;
                    }
                }
            }
        }

        for (j, row) in game.board.iter().rev().enumerate() {
            for (i, el) in row.iter().enumerate() {
                if *el == 1 {
                    let x = i as f32 * (CELL_SIZE + 2.0) + 8.0;
                    let y = j as f32 * (CELL_SIZE + 2.0) + 80.0;
                    draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, GREEN);
                }
            }
        }

        draw_text(format!("LEVEL: {}", level).as_str(), 8.0, 20.0, 20.0, GREEN);

        next_frame().await
    }
}
