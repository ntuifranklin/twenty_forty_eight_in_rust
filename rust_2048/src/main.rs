#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
extern crate rand;
extern crate termion;

mod board;
mod display;
mod foundation;
mod game_2048;

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{stdout, Write};
use std::thread;

use game_2048::GameStatus;

fn main() {
    let mut stdin = termion::async_stdin().events();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let board = board::Board::new();
    let mut game_2048 = game_2048::Game::new();

    display::display(&mut stdout, &board, &game_2048);
    stdout.flush().unwrap();

    loop {
        if let Some(evt) = stdin.next() {
            let mut changed = false;
            match evt.unwrap() {
                Event::Key(Key::Char('q')) => match game_2048.game_status() {
                    GameStatus::GameInterrupted | GameStatus::GameWon | GameStatus::GameLost => {
                        break
                    }
                    _ => {
                        game_2048.intrpt();
                    }
                },
                Event::Key(Key::Char('y')) => match game_2048.game_status() {
                    GameStatus::GameInterrupted | GameStatus::GameWon => break,
                    _ => (),
                },
                Event::Key(Key::Char('n')) => match game_2048.game_status() {
                    GameStatus::GameInterrupted | GameStatus::GameWon => game_2048.ongoing(),
                    _ => (),
                },
                Event::Key(key) => {
                    if let GameStatus::GameOngoing = game_2048.game_status() {
                        changed = game_2048.direction(key)
                    }
                }
                _ => (),
            };
            if changed {
                game_2048.new_square();
            } else {
                game_2048.checker();
            }
            display::display(&mut stdout, &board, &game_2048);
        };
        if game_2048.game_status() == game_2048::GameStatus::GameWon {
            display::display(&mut stdout, &board, &game_2048);
            thread::sleep(std::time::Duration::from_millis(150));
        } else {
            thread::sleep(std::time::Duration::from_millis(50));
        }
        stdout.flush().unwrap();
    }
}
