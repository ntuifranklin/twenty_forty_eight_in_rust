use board;
use game_2048;
use game_2048::GameStatus;
use std::io::Write;
use termion;

fn top<W>(out: &mut W, score: i32)
where
    W: Write,
{
    write!(out, "Game of 2048 {num:>pad$}\r\n", num = score, pad = 11).unwrap();
}

fn bottom<W>(out: &mut W, status: &GameStatus)
where
    W: Write,
{
    let text = match *status {
        GameStatus::GameOngoing => "    [ ← ↑ → ↓ ], q is to quit\r\n",
        GameStatus::GameLost => "    [  🎮 ⛔  ], q is to quit\r\n",
        GameStatus::GameInterrupted => "    [  🎮 🚦  ], do you want to quit? (y/n)\r\n",
        GameStatus::GameWon => "    [ 🎉🎉🎉 ], do you want to quit? (y/n)\r\n",
    };
    write!(out, "{}", text).unwrap();
}

fn clear<W>(out: &mut W)
where
    W: Write,
{
    write!(
        out,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Hide,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
}

pub fn display<W>(out: &mut W, board: &board::Board, game_2048: &game_2048::Game)
where
    W: Write,
{
    clear(out);
    top(out, game_2048.score());
    match game_2048.game_status() {
        GameStatus::GameOngoing => board.print(game_2048.number(), out),
        GameStatus::GameLost => board.print_lost(game_2048.number(), out),
        GameStatus::GameInterrupted => board.no_print(game_2048.number(), out),
        GameStatus::GameWon => board.print_won(game_2048.number(), out),
    };
    bottom(out, &game_2048.game_status());
}
