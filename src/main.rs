#![allow(dead_code)]
#[warn(unused_mut)]

use std::{io::Empty, thread::current};

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
    black_captures : usize,
    white_captures : usize,
    move_color : Cell, 
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
            move_color : Cell::Black,
        }
    }

    fn setup_game(&mut self) {
        self.black_captures = 0;
        self.white_captures = 0;
        self.board = [[Cell::Empty.clone(); 19]; 19];
        self.move_color = Cell::Black
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
            self.black_captures, self.white_captures, self.move_color.repr());
    }

    fn has_liberty(&self, x: &usize, y: &usize, mut checked: &Vec<(usize, usize)>) -> (bool, Vec<(usize, usize)>){
        let current_cell_color = self.board[*y][*x];
        checked.push((*x, *y));
        for k in 0..4 {
            let x_off: isize = *x as isize + [0, 1, 0, -1][k];
            let y_off: isize = *y as isize + [1, 0, -1, 0][k];
            if !in_bounds(x_off, y_off) {continue;}
            let x_off: usize = x_off as usize;
            let y_off: usize = y_off as usize;
            match self.board[y_off][x_off] {
                Cell::Empty => {return (true, checked);},
                other => if other == current_cell_color && !checked.contains(&(x_off, y_off)) {
                    let (has_liberty, marked) = self.has_liberty(&x_off, &y_off, checked);
                    if has_liberty {
                        return (true, marked);
                    } else {
                        for mark in marked {
                            if !checked.contains(&mark){
                                checked.push(mark);
                            }
                        }
                    }
                }
            }
        }
        (false, checked)
    }

    fn play_move(&mut self, x: &usize, y: &usize) -> bool {

        if self.board[*y][*x] != Cell::Empty {
            println!("Cannot place stone at {}, {}!", x, y);
            return false;
        }
        
        self.board[*y][*x] = self.move_color;
        self.repr();
        println!("{}", self.has_liberty(&1, &0, vec![]).0);
        // Check if the tentatively placed stone kills a group
        let mut is_kill: bool = false;
        for k in 0..4 {
            let x_off: isize = *x as isize + [0, 1, 0, -1][k];
            let y_off: isize = *y as isize + [1, 0, -1, 0][k];
            if !in_bounds(x_off, y_off) {continue;}
            let x_off: usize = x_off as usize;
            let y_off: usize = y_off as usize;
            if self.board[y_off][x_off] == change_color(self.move_color) {
                let (has_liberty, marked) = self.has_liberty(&x_off, &y_off, vec![]);
                if has_liberty { continue; }

                is_kill = true;
                if self.move_color == Cell::Black {
                    self.black_captures += marked.len();
                } else {
                    self.white_captures += marked.len();
                }
                for stone in marked {
                    self.board[stone.1][stone.0] = Cell::Empty;
                }
            }
        }
        if !is_kill {
            let (has_liberty, _) = self.has_liberty(&x, &y, vec![]);
            if !has_liberty {
                println!("Suicide Move at {}, {}!", x, y);
                self.board[*y][*x] = Cell::Empty;
                return false;
            }
        }
        println!("Played Move at {}, {}!", x, y);
        self.move_color = change_color(self.move_color);
        true
    }
}

fn main() {
    let mut b = Board::new();
    
    b.play_move(&2, &0);
    //b.repr();
    b.play_move(&3, &0);
    //b.repr();
    b.play_move(&1, &0);
    //b.repr();
    b.play_move(&2, &1);
    //b.repr();
    b.play_move(&13, &17);
    //b.repr();
    b.play_move(&1, &1);
    //b.repr();
    b.play_move(&0, &0);
    //b.repr();
    b.play_move(&0, &1);
    b.repr();
}
