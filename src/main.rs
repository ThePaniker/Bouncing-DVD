use rand::Rng;
use std::thread;
use std::time::Duration;
use std::process::Command;

const WIDTH: u32 = 50;
const HEIGHT: u32 = 30;

#[derive(Debug, PartialEq)]
struct Point(u32, u32);

#[derive(Debug)]
enum Direction {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

struct Dvd<'a> {
    logo: (char, char, char),
    position: Point,
    direction: &'a Direction,
}    

impl<'a> Dvd<'a> {
    fn update(&mut self) {
        match self.direction {
            Direction::UpLeft => {
                self.position.0 -= 1;
                self.position.1 -= 1;
                if self.position.0 == 2 {
                    self.direction = &Direction::UpRight;
                } else if self.position.1 == 1 {
                    self.direction = &Direction::DownLeft;
                }
            },

            Direction::UpRight => {
                self.position.0 += 1;
                self.position.1 -= 1;
                if self.position.0 == WIDTH - 3 {
                    self.direction = &Direction::UpLeft;
                } else if self.position.1 == 1 {
                    self.direction = &Direction::DownRight;
                }
            },

            Direction::DownLeft => {
                self.position.0 -= 1;
                self.position.1 += 1;
                if self.position.0 == 2 {
                    self.direction = &Direction::DownRight;
                } else if self.position.1 == HEIGHT - 2 {
                    self.direction = &Direction::UpLeft;
                }
            },

            Direction::DownRight => {
                self.position.0 += 1;
                self.position.1 += 1;
                if self.position.0 == WIDTH - 3 {
                    self.direction = &Direction::DownLeft;
                } else if self.position.1 == HEIGHT - 2 {
                    self.direction = &Direction::UpRight;
                }
            },
        }
    }
}

fn make_board() -> Vec<Point> {
    let mut board: Vec<Point> = vec![];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            board.push(Point(x, y));
        }
    }
    board
}

fn print_board(board: &Vec<Point>, dvd: &Dvd) {
    let d_pos = Point( dvd.position.0 - 1, dvd.position.1 );
    let v_pos = Point( dvd.position.0, dvd.position.1 );
    let d_pos2 = Point( dvd.position.0 + 1, dvd.position.1 );
    for point in board {
        if point == &d_pos {
            print!("{}", dvd.logo.0);
        } else if point == &v_pos {
            print!("{}", dvd.logo.1);
        } else if point == &d_pos2 {
            print!("{}", dvd.logo.2);
        } else if point.0 == WIDTH - 1 || point.1 == HEIGHT - 1 ||
           point.0 == 0 || point.1 == 0 {
            print!("#");
        } else {
            print!(" ");
        }
        if point.0 == WIDTH - 1 {
            println!();
        }
    }
}

fn get_cmd() -> String {
    if cfg!(target_os = "windows") {
        return String::from("cls");
    }
    String::from("clear")
}

fn main() {
    let directions: [Direction; 4] = [ 
        Direction::UpLeft,
        Direction::UpRight,
        Direction::DownLeft,
        Direction::DownRight,
    ];

    let mut dvd = Dvd {
        logo: ('D', 'V', 'D'),
        position: Point(
            rand::thread_rng().gen_range(3..WIDTH - 5),
            rand::thread_rng().gen_range(3..HEIGHT - 5),
        ),
        direction: &directions[rand::thread_rng().gen_range(0..3)],
    };

    let output = Command::new(get_cmd()).output().unwrap();
    let cls = String::from_utf8_lossy(&output.stdout);

    let board = make_board();
    loop {
        println!("{cls}");
        print_board(&board, &dvd);
        dvd.update();
        thread::sleep(Duration::from_secs_f64(0.2));
    }
}
