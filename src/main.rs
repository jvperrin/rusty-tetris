extern crate rand;
extern crate ncurses;
use ncurses::*;

#[derive(PartialEq)]
enum Status {
    SUCCESS,
    FAILURE,
}

enum Direction {
    LEFT,
    RIGHT,
    DOWN,
    NONE,
}

struct Piece {
    points: Vec<Point>,
}

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

struct Board {
    xmax: u32,
    ymax: u32,
    placed_pieces: Vec<Piece>,
    upcoming_pieces: Vec<Piece>,
}

fn get_next_move() -> Direction {
    let ch = getch();
    match ch {
        KEY_RIGHT => Direction::RIGHT,
        KEY_LEFT => Direction::LEFT,
        // KEY_DOWN => Direction::DOWN,
        _ => Direction::NONE,
    }
}

fn main() {
    initscr();
    cbreak();
    noecho();
    keypad(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    timeout(100);

    let mut xmax: i32 = 0;
    let mut ymax: i32 = 0;
    // Get screen size to get board limits
    getmaxyx(stdscr(), &mut ymax, &mut xmax);

    let mut board = Board {
        xmax: xmax as u32,
        ymax: ymax as u32,
        placed_pieces: vec![],
        upcoming_pieces: vec![],
    };

    let mut status = Status::SUCCESS;
    let mut dir = Direction::NONE;

    while status == Status::SUCCESS {
        clear();
        // TODO: Display the board
        refresh();
        dir = get_next_move();
        // TODO: Move the pieces down the board
    }
    endwin();
}
