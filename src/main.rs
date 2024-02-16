//#![allow(dead_code)]
//#[warn(unused_mut)]

#[derive(Copy, Clone, Debug)]
enum Cell {
    Empty,
    Black,
    White,
}

impl Cell {
    fn repr(&self) -> &str {
        match self {
            Cell::Empty => ".",
            Cell::Black => "X",
            Cell::White => "O",
        }
    }
}

fn change_color(cell: Cell) -> Cell {
    match cell {
        Cell::Empty => Cell::Empty,
        Cell::Black => Cell::White,
        Cell::White => Cell::Black,
    }
}

struct Board {
    board : [[Cell; 19]; 19],
    black_captures : u32,
    white_captures : u32,
    move_count : u32, 
}

fn in_bounds(x: i32, y: i32) -> bool {
    0 <= x && x < 19 && 0 <= y && y < 19
}

impl Board {
    fn new() -> Self {
        Board {
            
            board : [[Cell::Empty.clone(); 19]; 19],
            black_captures : 0,
            white_captures : 0,
            move_count : 0,
        }
    }

    fn setup_game(&mut self) {
        self.black_captures = 0;
        self.white_captures = 0;
        self.board = [[Cell::Empty.clone(); 19]; 19];
    }

    fn repr(&self) {
        let letters : &str = &"ABCDEFGHJKLMNOPQRST";

        print!("   ");
        for letter in letters.chars() {
            print!("{} ", letter);
        }
        print!("\n");

        for x in 0..19 {
            print!("{:>2} ", 19-x);
            for y in 0..19 {
                print!("{0} ", self.board[y][x].repr());
            }
            print!("{:<2}\n", 19-x);
        };
        
        print!("   ");
        for letter in letters.chars() {
            print!("{} ", letter);
        }
        print!("\n");

        print!("Captures: B: {0}, W: {1}\nMove: {2}\n", 
            self.black_captures, self.white_captures, self.move_count);
    }
}

fn main() {
    let mut b = Board::new();
    b.repr();
}
