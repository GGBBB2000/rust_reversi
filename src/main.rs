use std::io::Write;
extern crate ansi_term;

use ansi_term::Colour::*;

struct Board {
    board: [[u32; 8]; 8],
}

#[derive(Debug, Clone, Copy)]
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

const DIRS: [Dir; 8] = [
    Dir::UpperLeft,
    Dir::Up,
    Dir::UpperRight,
    Dir::Right,
    Dir::LowerRight,
    Dir::Low,
    Dir::LowerLeft,
    Dir::Left,
];

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

    fn game_over(&self) -> bool {
        //1:black 2:white
        let mut black = 0;
        let mut white = 0;
        for y in 0..8 {
            for x in 0..8 {
                if self.can_put(1, x, y) || self.can_put(2, x, y) {
                    return false;
                }
                if self.board[y][x] == 1 {
                    black += 1;
                } else if self.board[y][x] == 2 {
                    white += 1;
                }
            }
        }

        if black > white {
            println!("Black Win!");
        } else if black == white {
            println!("Draw");
        } else {
            println!("White Win!");
        }
        println!("Black: {} White: {}", black, white);

        true
    }

    fn print_board(&self, rock: u32) {
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
                } else if self.can_put(rock, x, y) {
                    print!("{}", Red.on(Green).paint("*"));
                } else {
                    print!("{}", Black.on(Green).paint("*"));
                }
            }
            println!();
        }
    }

    fn can_put(&self, rock: u32, x: usize, y: usize) -> bool {
        for d in DIRS.iter() {
            if self.search_board(rock, x, y, *d) != (x, y) {
                return true;
            }
        }
        false
    }

    fn put_rock(&mut self, rock: u32, x: usize, y: usize) {
        for d in DIRS.iter() {
            let target = self.search_board(rock, x, y, *d);
            self.reverse_rock(rock, x, y, target, *d);
            // println!("{:?}:{}:{}", d, target.0, target.1);
        }
    }

    fn search_board(&self, rock: u32, x: usize, y: usize, dir: Dir) -> (usize, usize) {
        match dir {
            Dir::UpperLeft => {
                if x == 0 || y == 0 {
                    return (x, y);
                }
                let mut x_i = x - 1;
                let mut y_i = y - 1;
                loop {
                    if self.board[y_i][x_i] == rock {
                        if x - 1 == x_i && y - 1 == y_i {
                            return (x, y);
                        }
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
            }
            Dir::Up => {
                if y == 0 {
                    return (x, y);
                }
                for i in (0..y).rev() {
                    if self.board[i][x] == rock {
                        if i == y - 1 {
                            return (x, y);
                        }
                        return (x, i);
                    } else if self.board[i][x] == 0 {
                        return (x, y);
                    }
                }
                (x, y)
            }
            Dir::UpperRight => {
                if x == 7 || y == 0 {
                    return (x, y);
                }
                let mut x_i = x + 1;
                let mut y_i = y - 1;
                loop {
                    if self.board[y_i][x_i] == rock {
                        if x_i == x + 1 && y_i == y - 1 {
                            return (x, y);
                        }
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
            }
            Dir::Right => {
                for i in x + 1..8 {
                    if self.board[y][i] == rock {
                        if i == x + 1 {
                            return (x, y);
                        }
                        return (i, y);
                    } else if self.board[y][i] == 0 {
                        return (x, y);
                    }
                }
                (x, y)
            }
            Dir::LowerRight => {
                if x == 7 || y == 7 {
                    return (x, y);
                }
                let mut x_i = x + 1;
                let mut y_i = y + 1;
                loop {
                    if self.board[y_i][x_i] == rock {
                        if x_i == x + 1 && y_i == y + 1 {
                            return (x, y);
                        }
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
            }
            Dir::Low => {
                for i in y + 1..8 {
                    if self.board[i][x] == rock {
                        if i == y + 1 {
                            return (x, y);
                        }
                        return (x, i);
                    } else if self.board[i][x] == 0 {
                        return (x, y);
                    }
                }
                (x, y)
            }
            Dir::LowerLeft => {
                if x == 0 || y == 7 {
                    return (x, y);
                }
                let mut x_i = x - 1;
                let mut y_i = y + 1;
                loop {
                    if self.board[y_i][x_i] == rock {
                        if x_i == x - 1 && y_i == y + 1 {
                            return (x, y);
                        }
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
            }
            Dir::Left => {
                for i in (0..x).rev() {
                    if self.board[y][i] == rock {
                        if i == x - 1 {
                            return (x, y);
                        }
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
        if x == target.0 && y == target.1 {
            return;
        }
        match dir {
            Dir::UpperLeft => {
                let mut x_i = x;
                let mut y_i = y;
                while x_i >= target.0 && y_i >= target.1 {
                    self.board[y_i][x_i] = rock;
                    if x_i == 0 || y_i == 0 {
                        break;
                    }
                    x_i -= 1;
                    y_i -= 1;
                }
            }
            Dir::Up => {
                for i in target.1..=y {
                    self.board[i][target.0] = rock;
                }
            }
            Dir::UpperRight => {
                let mut x_i = x;
                let mut y_i = y;
                while x_i <= target.0 && y_i >= target.1 {
                    self.board[y_i][x_i] = rock;
                    if x_i == 7 || y_i == 0 {
                        break;
                    }
                    x_i += 1;
                    y_i -= 1;
                }
            }
            Dir::Right => {
                for i in x..=target.0 {
                    self.board[target.1][i] = rock;
                }
            }
            Dir::LowerRight => {
                let mut x_i = x;
                let mut y_i = y;
                while x_i <= target.0 && y_i <= target.1 {
                    self.board[y_i][x_i] = rock;
                    if x_i == 7 || y_i == 7 {
                        break;
                    }
                    x_i += 1;
                    y_i += 1;
                }
            }
            Dir::Low => {
                for i in y..=target.1 {
                    self.board[i][target.0] = rock;
                }
            }
            Dir::LowerLeft => {
                let mut x_i = x;
                let mut y_i = y;
                while x_i >= target.0 && y_i <= target.1 {
                    self.board[y_i][x_i] = rock;
                    if x_i == 0 || y_i == 7 {
                        break;
                    }
                    x_i -= 1;
                    y_i += 1;
                }
            }
            Dir::Left => {
                for i in target.0..=x {
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
        if board.game_over() {
            break;
        }
        if rock == 1 {
            println!("Player:Black");
        } else {
            println!("Player:White");
        }
        board.print_board(rock);
        {
            let mut no_place_to_put = true;
            for y in 0..8 {
                for x in 0..8 {
                    if board.can_put(rock, x, y) && board.board[y][x] == 0 {
                        no_place_to_put = false;
                    }
                }
            }
            if no_place_to_put {
                println!("There isn't the place you can put the rock");
                rock = if rock % 2 == 0 { 1 } else { 2 };
                continue;
            }
        }
        let x = get_input("x");
        let y = get_input("y");
        if x > 7 || y > 7 {
            println!("Invalid value.");
            continue;
        } else if board.board[y][x] == 1 || board.board[y][x] == 2 {
            println!("There is already another rock!");
            continue;
        } else if !board.can_put(rock, x, y) {
            println!("{}", Red.paint("You can't put the rock there."));
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
