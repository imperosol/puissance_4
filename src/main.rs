use std::process::Command;

const VICTORY: u32 = 1;
const DEFEAT: u32 = 0;
const MAX_DEPTH: u32 = 31;
const GRID_SIZE: usize = 7;

const IMPOSSIBLE: u8 = 1;
const EMPTY: u8 = 2;
const OCCUPIED: u8 = 3;

const NORTH: u8 = 1;
const WEST: u8 = 2;
const SOUTH: u8 = 3;
const EAST: u8 = 4;

struct Movement {
    coords: [usize; 2],
    direction: u8,
}


fn main() {
    use std::time::Instant;
    let now = Instant::now();
    {
        let mut grid = create_grid();
        display_grid(&grid);
        game_recursion(&mut grid, 0);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

fn game_recursion(grid: &mut [[u8; 7]; 7], depth: u32) -> u32 {
    if depth == MAX_DEPTH {
        println!("Solution trouvée");
        display_grid(grid);
        return VICTORY;
    } else {
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                for direction in NORTH..=EAST { // eq. to 1..=4
                    let movement = Movement { coords: [row, col], direction };
                    if is_movable(grid, &movement) {
                        put_piece(grid, &movement);
                        let result: u32 = game_recursion(grid, depth + 1);
                        undo_move(grid, &movement);
                        if result == VICTORY {
                            display_grid(grid);
                            return VICTORY;
                        }
                    }
                }
            }
        }
    }
    DEFEAT
}

fn is_movable(grid: &[[u8; 7]; 7], wanted_move: &Movement) -> bool {
    let (row, col) = (wanted_move.coords[0], wanted_move.coords[1]);
    if grid[row][col] != OCCUPIED {
        return false;
    }
    let is_move_possible: bool = match wanted_move.direction {
        NORTH => row > 1
            && grid[row - 1][col] == OCCUPIED
            && grid[row - 2][col] == EMPTY,
        SOUTH => row < 5
            && grid[row + 1][col] == OCCUPIED
            && grid[row + 2][col] == EMPTY,
        EAST => col < 5
            && grid[row][col + 1] == OCCUPIED
            && grid[row][col + 2] == EMPTY,
        WEST => col > 1
            && grid[row][col - 1] == OCCUPIED
            && grid[row][col - 2] == EMPTY,
        _ => panic!()
    };
    is_move_possible
}

fn undo_move(grid: &mut [[u8; 7]; 7], m: &Movement) {
    let (old_row, old_col) = (m.coords[0], m.coords[1]);
    grid[old_row][old_col] = OCCUPIED;
    match m.direction {
        NORTH => {
            grid[old_row - 1][old_col] = OCCUPIED;
            grid[old_row - 2][old_col] = EMPTY;
        }
        SOUTH => {
            grid[old_row + 1][old_col] = OCCUPIED;
            grid[old_row + 2][old_col] = EMPTY;
        }
        WEST => {
            grid[old_row][old_col - 1] = OCCUPIED;
            grid[old_row][old_col - 2] = EMPTY;
        }
        EAST => {
            grid[old_row][old_col + 1] = OCCUPIED;
            grid[old_row][old_col + 2] = EMPTY;
        }
        _ => {}
    }
}

fn put_piece(grid: &mut [[u8; 7]; 7], m: &Movement) {
    let (row, col) = (m.coords[0], m.coords[1]);
    grid[row][col] = EMPTY;
    match m.direction {
        NORTH => {
            grid[row - 1][col] = EMPTY;
            grid[row - 2][col] = OCCUPIED;
        }
        SOUTH => {
            grid[row + 1][col] = EMPTY;
            grid[row + 2][col] = OCCUPIED;
        }
        WEST => {
            grid[row][col - 1] = EMPTY;
            grid[row][col - 2] = OCCUPIED;
        }
        EAST => {
            grid[row][col + 1] = EMPTY;
            grid[row][col + 2] = OCCUPIED;
        }
        _ => {}
    }
}

fn create_grid() -> [[u8; 7]; 7] {
    let mut grid: [[u8; GRID_SIZE]; GRID_SIZE] = [[OCCUPIED; GRID_SIZE]; GRID_SIZE];
    for row in 0..2 {
        for column in 0..2 {
            grid[row][column] = IMPOSSIBLE;
            grid[row + 5][column] = IMPOSSIBLE;
            grid[row][column + 5] = IMPOSSIBLE;
            grid[row + 5][column + 5] = IMPOSSIBLE;
        }
    }
    grid[3][3] = EMPTY;
    grid
}

fn display_grid(grid: &[[u8; 7]; 7]) {
    for row in grid.iter() {
        for &column in row.iter() {
            match column {
                IMPOSSIBLE => print!("  "),
                EMPTY => print!("_ "),
                OCCUPIED => print!("X "),
                _ => {}
            }
        }
        println!();
    }
    println!()
}


