extern crate rand;

use foundation;
use rand::{thread_rng, Rng};
use termion::event::Key;

#[derive(Clone, PartialEq)]
pub enum GameStatus {
    GameOngoing,
    GameWon,
    GameLost,
    GameInterrupted,
}

#[derive(Clone)]
pub struct Game {
    status: GameStatus,
    just_won: bool,
    score: i32,
    num: [i32; 16],
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Game {
    pub fn new() -> Game {
        let mut rng = thread_rng();
        let mut num = [0; 16];
        num[0] = 1;
        num[1] = 1;
        rng.shuffle(&mut num);
        Game {
            status: GameStatus::GameOngoing,
            just_won: false,
            score: 0,
            num,
        }
    }

    pub fn game_status(&self) -> GameStatus {
        self.status.clone()
    }
    pub fn intrpt(&mut self) {
        self.status = GameStatus::GameInterrupted;
    }
    pub fn ongoing(&mut self) {
        self.status = GameStatus::GameOngoing;
    }
    pub fn checker(&mut self) {
        let mut copy = self.clone();
        if !(copy.right() || copy.left() || copy.up() || copy.down()) {
            self.status = GameStatus::GameLost;
        }
    }
    pub fn number(&self) -> [i32; 16] {
        self.num
    }
    pub fn score(&self) -> i32 {
        self.score
    }

    fn horizontal_dir(&mut self, dir: Direction) -> bool {
        let mut mutated = false;
        let mut score = 0;
        let mut won = false;
        self.num
            .chunks_mut(4)
            .map(|row| {
                let (new_row, new_score) = match dir {
                    Direction::Right => foundation::right_slider(row),
                    Direction::Left => foundation::left_slider(row),
                    _ => (row.to_vec(), 0),
                };
                if new_score == 2048 {
                    won = true;
                }
                score += new_score;
                for i in 0..4 {
                    if row[i] != new_row[i] {
                        row[i] = new_row[i];
                        mutated = true;
                    }
                }
            })
            .collect::<Vec<_>>();
        self.score += score;
        if won && !self.just_won {
            self.status = GameStatus::GameWon;
            self.just_won = true;
        }
        mutated
    }
    fn vertical_dir(&mut self, dir: Direction) -> bool {
        foundation::transpose(&mut self.num);
        let mutated = match dir {
            Direction::Up => self.left(),
            Direction::Down => self.right(),
            _ => false,
        };
        foundation::transpose(&mut self.num);
        mutated
    }
    pub fn new_square(&mut self) {
        let value = if rand::random::<i32>() % 10 == 1 {
            2
        } else {
            1
        };

        let zeroes_index = self
            .num
            .iter()
            .enumerate()
            .filter(|&(_, x)| *x == 0)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        self.num[zeroes_index[rand::random::<usize>() % zeroes_index.len()]] = value;
    }
    pub fn up(&mut self) -> bool {
        self.vertical_dir(Direction::Up)
    }
    pub fn down(&mut self) -> bool {
        self.vertical_dir(Direction::Down)
    }
    pub fn right(&mut self) -> bool {
        self.horizontal_dir(Direction::Right)
    }
    pub fn left(&mut self) -> bool {
        self.horizontal_dir(Direction::Left)
    }

    pub fn direction(&mut self, key: Key) -> bool {
        match key {
            Key::Up => self.up(),
            Key::Left => self.left(),
            Key::Right => self.right(),
            Key::Down => self.down(),
            _ => false,
        }
    }
}
