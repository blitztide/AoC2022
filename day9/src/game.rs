const BOARD_SIZE: usize = 100;
const ORIGIN: i32 = 50;

pub struct State {
    head: Position,
    tail: Position,
    board: Board,
}

impl State {
    pub fn new() -> State {
        let origin: Position = Position { x:ORIGIN, y:ORIGIN};
        State {head: origin, tail: origin, board: [[0;BOARD_SIZE];BOARD_SIZE]}
    }

    pub fn update(&mut self, instruction: &Instruction) {
        match instruction.direction {
            Direction::Up => return,
            Direction::Down => return,
            Direction::Left => return,
            Direction::Right => return,
        };
    }

    pub fn rope_travel(&self) {
        println!("{}",self.head.x);
    }

}

type Board = [[i32;BOARD_SIZE];BOARD_SIZE];
#[derive(Clone,Copy)]
struct Position {
    x: i32,
    y: i32,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
pub struct Instruction {
    pub direction: Direction,
    pub count: i32,
}

impl Instruction {
    pub fn new(input: &str) -> Instruction {
        let x = input.split_once(" ").unwrap();
        let count = x.1.parse::<i32>().unwrap();
        match x.0 {
            "L" => Instruction { direction: Direction::Left, count: count},
            "R" => Instruction { direction: Direction::Right, count: count},
            "U" => Instruction { direction: Direction::Up, count: count},
            "D" => Instruction { direction: Direction::Down, count: count},
            _ => panic!("Not a valid instruction"),
        }
        
    }
}

pub type Instructions = Vec<Instruction>;