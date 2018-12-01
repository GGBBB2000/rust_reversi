use std::io::Write;
extern crate ansi_term;

use ansi_term::Colour::*;

struct Board {
    board: [[u32; 8]; 8],
}

enum Dir {
    UpperLeft,
    Up,
    UpperRight,
    Right,
    LowerRight,
    Low,
    LowerLeft,
    Left,
}

impl Board {
    fn new() -> Self {
        let mut board = [[0; 8]; 8];
        board[3][3] = 2;
        board[4][4] = 2;
        board[3][4] = 1;
        board[4][3] = 1;
        // Self { board:board }
        Self { board }
    }

    fn print_board(&self) {
        print!(" ");
        for i in 0..8 {
            print!("{}", i);
        }
        println!();
        for y in 0..8 {
            print!("{}", y);
            for x in 0..8 {
                if self.board[y][x] == 1 {
                    print!("{}", Black.on(Green).paint("●"));
                } else if self.board[y][x] == 2 {
                    print!("{}", White.on(Green).paint("●"));
                } else {
                    print!("{}", Black.on(Green).paint("*"));
                }
            }
            println!();
        }
    }

    fn put_rock(&mut self, rock: u32, x: usize, y: usize) {
        let target = self.search_board(rock, x, y, Dir::UpperLeft);
        self.reverse_rock(rock, x, y, target, Dir::UpperLeft);
        println!("UpperLeft:{}:{}", target.0, target.1);

        let target = self.search_board(rock, x, y, Dir::Up);
        self.reverse_rock(rock, x, y, target, Dir::Up);
        println!("Up:{}:{}", target.0, target.1);

        let target = self.search_board(rock, x, y, Dir::UpperRight);
        self.reverse_rock(rock, x, y, target, Dir::UpperRight);
        println!("UpperRight:{}:{}", target.0, target.1);

        let target = self.search_board(rock, x, y, Dir::Right);
        self.reverse_rock(rock, x, y, target, Dir::Right);
        println!("Right:{}:{}", target.0, target.1);
        
        let target = self.search_board(rock, x, y, Dir::LowerRight);
        self.reverse_rock(rock, x, y, target, Dir::LowerRight);
        println!("LowerRight:{}:{}", target.0, target.1);

        let target = self.search_board(rock, x, y, Dir::Low);
        self.reverse_rock(rock, x, y, target, Dir::Low);
        println!("Low:{}:{}", target.0, target.1);
        
        let target = self.search_board(rock, x, y, Dir::LowerLeft);
        self.reverse_rock(rock, x, y, target, Dir::LowerLeft);
        println!("LowerLeft:{}:{}", target.0, target.1);

        let target = self.search_board(rock, x, y, Dir::Left);
        self.reverse_rock(rock, x, y, target, Dir::Left);
        println!("Left:{}:{}\n", target.0, target.1);
    }

    fn search_board(&self, rock: u32, x: usize, y: usize, dir: Dir) -> (usize, usize) {
        match dir {
            Dir::UpperLeft => {
                if x == 0 || y == 0{
                    return (x, y);
                }
                let mut x_i = x - 1;
                let mut y_i = y - 1;
                loop {
                    if self.board[y_i][x_i] == rock {
                        return (x_i, y_i);
                    } else if self.board[y_i][x_i] == 0 {
                        return (x, y);
                    }
                    if x_i == 0 || y_i == 0 {
                        return (x, y);
                    }
                    x_i -= 1;
                    y_i -= 1;
                }
            },
            Dir::Up => {
                if y == 0 {
                    return (x, y);
                }
                for i in (0..y).rev() {
                    if self.board[i][x] == rock {
                        return (x, i);
                    } else if self.board[i][x] == 0 {
                        return (x, y);
                    }
                }
                (x, y)
            }
            Dir::UpperRight => {
                if x == 7 || y == 0{
                    return (x, y);
                }
                let mut x_i = x + 1;
                let mut y_i = y - 1;
                loop {
                    if self.board[y_i][x_i] == rock {
                        return (x_i, y_i);
                    } else if self.board[y_i][x_i] == 0 {
                        return (x, y);
                    }
                    if x_i == 7 || y_i == 0 {
                        return (x, y);
                    }
                    x_i += 1;
                    y_i -= 1;
                }
            },
            Dir::Right => {
                for i in x + 1..8 {
                    if self.board[y][i] == rock {
                        return (i, y);
                    } else if self.board[y][i] == 0 {
                        return (x, y);
                    }
                }
                (x, y)
            },
            Dir::LowerRight => {
                if x == 7 || y == 7{
                    return (x, y);
                }
                let mut x_i = x + 1;
                let mut y_i = y + 1;
                loop {
                    if self.board[y_i][x_i] == rock {
                        return (x_i, y_i);
                    } else if self.board[y_i][x_i] == 0 {
                        return (x, y);
                    }
                    if x_i == 7 || y_i == 7 {
                        return (x, y);
                    }
                    x_i += 1;
                    y_i += 1;
                }
            },
            Dir::Low => {
                for i in y + 1..8 {
                    if self.board[i][x] == rock {
                        return (x, i);
                    } else if self.board[i][x] == 0 {
                        return (x, y);
                    }
                }
                (x, y)
            },
            Dir::LowerLeft => {
                if x == 0 || y == 7{
                    return (x, y);
                }
                let mut x_i = x - 1;
                let mut y_i = y + 1;
                loop {
                    if self.board[y_i][x_i] == rock {
                        return (x_i, y_i);
                    } else if self.board[y_i][x_i] == 0 {
                        return (x, y);
                    }
                    if x_i == 0 || y_i == 7 {
                        return (x, y);
                    }
                    x_i -= 1;
                    y_i += 1;
                }
            },
            Dir::Left => {
                for i in (0..x).rev() {
                    if self.board[y][i] == rock {
                        return (i, y);
                    } else if self.board[y][i] == 0 {
                        return (x, y);
                    }
                }
                (x, y)
            }
        }
    }

    fn reverse_rock(&mut self, rock: u32, x: usize, y: usize, target: (usize, usize), dir: Dir) {
        match dir {
            Dir::UpperLeft => {
                let mut x_i = x;
                let mut y_i = y;
                while x_i >= target.0 && y >= target.1 {
                    self.board[y_i][x_i] = rock;
                    if x_i == 0 || y == 0 {
                        break;
                    }
                    x_i -= 1;
                    y_i -= 1;
                }
            },
            Dir::Up => {
                for i in target.1..y + 1 {
                    self.board[i][target.0] = rock;
                }
            },
            Dir::UpperRight => {
                let mut x_i = x;
                let mut y_i = y;
                while x_i <= target.0 && y >= target.1 {
                    self.board[y_i][x_i] = rock;
                    if x_i == 7 || y == 0 {
                        break;
                    }
                    x_i += 1;
                    y_i -= 1;
                }
            },
            Dir::Right => {
                for i in x..target.0 + 1 {
                    self.board[target.1][i] = rock;
                }
            },
            Dir::LowerRight => {
                let mut x_i = x;
                let mut y_i = y;
                while x_i <= target.0 && y <= target.1 {
                    self.board[y_i][x_i] = rock;
                    if x_i == 7 || y == 7 {
                        break;
                    }
                    x_i += 1;
                    y_i += 1;
                }
            },
            Dir::Low => {
                for i in y..target.1 + 1 {
                    self.board[i][target.0] = rock;
                }
            },
            Dir::LowerLeft => {
                let mut x_i = x;
                let mut y_i = y;
                while x_i >= target.0 && y >= target.1 {
                    self.board[y_i][x_i] = rock;
                    if x_i == 0 || y == 7 {
                        break;
                    }
                    x_i -= 1;
                    y_i += 1;
                }
            },
            Dir::Left => {
                for i in target.0..x + 1{
                    self.board[target.1][i] = rock;
                }
            }
        };
    }
}

fn main() {
    play_game();
}

fn play_game() {
    //none 0 black 1 white 2
    let mut board = Board::new();
    let mut rock = 1;

    loop {
        if rock == 1 {
            println!("Player:Black");
        } else {
            println!("Player:White");
        }
        board.print_board();
        let x = get_input("x");
        let y = get_input("y");
        if x > 7 || y > 7 {
            println!("Invalid value");
            continue;
        }
        board.put_rock(rock, x, y);
        rock = if rock % 2 == 0 { 1 } else { 2 };
    }
}

fn get_input(arg: &str) -> usize {
    print!("{}:", arg);
    std::io::stdout().flush().unwrap();
    let mut x = String::new();
    std::io::stdin().read_line(&mut x).unwrap();
    x.trim().parse().unwrap()
}
