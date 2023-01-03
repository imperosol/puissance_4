use std::time::Instant;
use std::process::Command;
use std::slice::{Iter};
use crate::CellState::{Empty, Occupied};
use crate::Direction::*;

const MAX_DEPTH: u32 = 31;
const GRID_SIZE: usize = 7;

#[derive(Debug, Hash, Eq, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [North, South, East, West];
        DIRECTIONS.iter()
    }
}

#[derive(PartialEq, Copy, Clone)]
enum CellState {
    Empty,
    Occupied,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Movement {
    coords: [usize; 2],
    direction: &'static Direction,
}

type Grid = [[Option<CellState>; GRID_SIZE]; GRID_SIZE];


fn main() {
    let now = Instant::now();
    {
        let mut grid = create_grid();
        display_grid(&grid);
        game_recursion(&mut grid, 0);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    Command::new("cmd.exe").arg("/c").arg("pause").status().unwrap();
}

fn game_recursion(grid: &mut Grid, depth: u32) -> bool {
    if depth == MAX_DEPTH {
        println!("Solution trouvée");
        display_grid(grid);
        return true;
    }
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            for direction in Direction::iterator() {
                let movement = Movement { coords: [row, col], direction };
                if is_movable(grid, &movement) {
                    put_piece(grid, &movement);
                    let result = game_recursion(grid, depth + 1);
                    undo_move(grid, &movement);
                    if result {
                        display_grid(grid);
                        return true;
                    }
                }
            }
        }
    }
    false
}

#[inline(always)]
fn is_movable(grid: &Grid, wanted_move: &Movement) -> bool {
    let (row, col) = (wanted_move.coords[0], wanted_move.coords[1]);
    if grid[row][col] != Some(Occupied) {
        return false;
    }
    match wanted_move.direction {
        North => row > 1
            && grid[row - 1][col] == Some(Occupied)
            && grid[row - 2][col] == Some(Empty),
        South => row < 5
            && grid[row + 1][col] == Some(Occupied)
            && grid[row + 2][col] == Some(Empty),
        East => col < 5
            && grid[row][col + 1] == Some(Occupied)
            && grid[row][col + 2] == Some(Empty),
        West => col > 1
            && grid[row][col - 1] == Some(Occupied)
            && grid[row][col - 2] == Some(Empty),
    }
}

#[inline(always)]
fn undo_move(grid: &mut Grid, m: &Movement) {
    let (old_row, old_col) = (m.coords[0], m.coords[1]);
    grid[old_row][old_col] = Some(Occupied);
    match m.direction {
        North => {
            grid[old_row - 1][old_col] = Some(Occupied);
            grid[old_row - 2][old_col] = Some(Empty);
        }
        South => {
            grid[old_row + 1][old_col] = Some(Occupied);
            grid[old_row + 2][old_col] = Some(Empty);
        }
        West => {
            grid[old_row][old_col - 1] = Some(Occupied);
            grid[old_row][old_col - 2] = Some(Empty);
        }
        East => {
            grid[old_row][old_col + 1] = Some(Occupied);
            grid[old_row][old_col + 2] = Some(Empty);
        }
    }
}

#[inline(always)]
fn put_piece(grid: &mut Grid, m: &Movement) {
    let (row, col) = (m.coords[0], m.coords[1]);
    grid[row][col] = Some(Empty);
    match m.direction {
        North => {
            grid[row - 1][col] = Some(Empty);
            grid[row - 2][col] = Some(Occupied);
        }
        South => {
            grid[row + 1][col] = Some(Empty);
            grid[row + 2][col] = Some(Occupied);
        }
        West => {
            grid[row][col - 1] = Some(Empty);
            grid[row][col - 2] = Some(Occupied);
        }
        East => {
            grid[row][col + 1] = Some(Empty);
            grid[row][col + 2] = Some(Occupied);
        }
    }
}

#[inline(always)]
fn create_grid() -> Grid {
    let mut grid = [[Some(Occupied); GRID_SIZE]; GRID_SIZE];
    for row in 0..2 {
        for column in 0..2 {
            grid[row][column] = None;
            grid[row + 5][column] = None;
            grid[row][column + 5] = None;
            grid[row + 5][column + 5] = None;
        }
    }
    grid[3][3] = Some(Empty);
    grid
}

fn display_grid(grid: &Grid) {
    let mut buffer = String::new();
    for row in grid.iter() {
        for column in row.iter() {
            match column {
                None => buffer.push_str("  "),
                Some(state) => match state {
                    Empty => buffer.push_str("_ "),
                    Occupied => buffer.push_str("X ")
                }
            }
        }
        buffer.push('\n');
    }
    println!("{}", buffer);
}

