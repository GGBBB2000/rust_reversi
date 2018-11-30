use std::io::Write;

struct Board {
    board : [[u32; 8]; 8]
}

enum Dir {
    UpperLeft,
    Up,
    UpperRight,
    Right,
    LowerRight,
    Low,
    LowerLeft,
    Left
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
        for y in 0..8 {
            for x in 0..8 {
                print!("{} ", self.board[y][x]);
            }
            println!();
        }
    }

    fn put_rock(&self, rock: u32, x: usize, y: usize) {
        let target = self.search_board(rock, x, y, Dir::Up);
        println!("{}:{}", target.0, target.1);
        // board[y][x] = rock;
    }

    fn search_board(
        &self,
        rock: u32,
        x: usize,
        y: usize,
        dir: Dir,
        ) -> (usize, usize) {
        /*0 1 2
         *  ↑
         *3←o→4
            ↓
          5 6 7*/

        match dir {
            Dir::UpperLeft => (0, 0),
            Dir::Up => {
                for i in (0..y + 2).rev() {
                    if self.board[i][x] == rock {
                        return (x, i);
                    }
                }
                (x, y)
            }
            Dir::UpperRight => (0, 0),
            Dir::Right => (0, 0),
            Dir::LowerRight => (0, 0),
            Dir::Low => (0, 0),
            Dir::LowerLeft => {
                for i in y + 1..7 {
                    if self.board[i][x] == rock {
                        return (x, i);
                    }
                }
                (x, y)
            }
            Dir::Left => (0, 0),
        }
    }
}

fn main() {
    play_game();
}


fn play_game() {
    //none 0 black 1 white 2
    let board = Board::new();
    let mut rock = 1;

    loop {
        println!("Player:{}", rock);
        board.print_board();
        let x = get_input("x");
        let y = get_input("y");
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
