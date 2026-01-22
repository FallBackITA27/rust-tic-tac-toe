use std::hint::unreachable_unchecked;

use getch_rs::Key;

use crate::cellstate::{CellState, Direction, GameMatrix, WinState};

pub mod cellstate;

fn main() {
    // use std::random() when out of nightly https://github.com/rust-lang/rust/issues/130703

    let mut game_matrix = GameMatrix::<3>::default();

    let mut player_turn = CellState::from((rand::random::<u8>() % 2) + 1);
    let mut win = WinState::Unfinished;

    while win == WinState::Unfinished {
        println!("{player_turn} Player Turn");

        if CellState::Cross == player_turn {
            let (mut r, mut c) = game_matrix.first_free_cell(0, 0, Direction::Right);
            let _ = game_matrix.write_to_cell(r, c, CellState::Hover);

            println!("{game_matrix}");
            let getch = getch_rs::Getch::new();
            loop {
                let direction = match getch.getch() {
                    Ok(Key::Esc) => return,
                    Ok(Key::Char(' ') | Key::Insert | Key::Char('\r') | Key::Char('\n')) => {
                        println!("{game_matrix}");
                        println!("Enters here!");
                        break;
                    }
                    Ok(Key::Up | Key::Char('k')) => Direction::Up,
                    Ok(Key::Down | Key::Char('j')) => Direction::Down,
                    Ok(Key::Left | Key::Char('h')) => Direction::Left,
                    Ok(Key::Right | Key::Char('l')) => Direction::Right,
                    Err(e) => {
                        eprintln!("{e}");
                        continue;
                    }
                    _ => continue,
                };

                let (old_row, old_col) = (r, c);
                (r, c) = game_matrix.first_free_cell(r, c, direction);
                let _ = game_matrix.write_to_cell(old_row, old_col, CellState::Empty);
                let _ = game_matrix.write_to_cell(r, c, CellState::Hover);
                println!("{game_matrix}");
            }

            let _ = game_matrix.write_to_cell(r, c, player_turn);
            player_turn = CellState::Circle;
        } else {
            println!("{game_matrix}");
            let mut r = usize::MAX;
            let mut c = usize::MAX;
            while !game_matrix.cell_free(r, c) {
                (r, c) = (
                    (rand::random::<u64>() % (game_matrix.size() as u64)) as usize,
                    (rand::random::<u64>() % (game_matrix.size() as u64)) as usize,
                );
            }

            let _ = game_matrix.write_to_cell(r, c, player_turn);
            player_turn = CellState::Cross;
        }

        win = game_matrix.check_win();
    }

    match win {
        WinState::Tie => println!("It's a tie"),
        WinState::Win => println!("You win!"),
        WinState::Loss => println!("You lose!"),
        WinState::Unfinished => unsafe { unreachable_unchecked() },
    }
    println!("{game_matrix}");
}
