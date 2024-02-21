//#![allow(dead_code)]
//#[warn(unused_mut)]

#[derive(Copy, Clone, Debug, PartialEq)]
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

fn in_bounds(x: isize, y: isize) -> bool {
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
            print!("{:>2} ", x+1);
            for y in 0..19 {
                print!("{0} ", self.board[y][x].repr());
            }
            print!("{:<2}\n", x+1);
        };
        
        print!("   ");
        for letter in letters.chars() {
            print!("{} ", letter);
        }
        print!("\n");

        print!("Captures: B: {0}, W: {1}\nMove: {2}\n", 
            self.black_captures, self.white_captures, self.move_count);
    }

    fn has_liberty(&self, x: &usize, y: &usize, mut checked: Vec<(usize, usize)>) -> bool{
        let current_cell_color = self.board[*y][*x];
        checked.push((*x, *y));
        for k in 0..4 {
            let x_off: isize = *x as isize + [0, 1, 0, -1][k];
            let y_off: isize = *y as isize + [1, 0, -1, 0][k];
            if !in_bounds(x_off, y_off) {continue;}
            let x_off: usize = x_off as usize;
            let y_off: usize = y_off as usize;
            match self.board[y_off][x_off] {
                Cell::Empty => {return true;},
                other => if other == current_cell_color && !checked.contains(&(x_off, y_off)) {
                    return self.has_liberty(&x_off, &y_off, checked);
                },
            }
        }
        false
    }
}

fn main() {
    let mut b = Board::new();
    b.board[0][2] = Cell::Black;
    b.board[0][1] = Cell::Black;
    b.board[0][3] = Cell::White;
    b.board[1][2] = Cell::White;
    b.board[1][1] = Cell::White;
    b.repr();
    b.has_liberty(&2, &0, vec![]);
    println!("{}", b.has_liberty(&2, &0, vec![]));
}
