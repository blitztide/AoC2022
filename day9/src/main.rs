mod game;
use game::{State,Instruction,Instructions};
const INPUT: &str = include_str!("input.txt");

fn main() {
    // Tail touches head problem
    let (mut state,instructions) = init();
    problem1(state,&instructions);
    // Clear state
    state = State::new();
    problem2(state,&instructions);
    println!("");
}

fn init() -> (State,Instructions) {
    let state: State = State::new();
    let mut instructions: Instructions = Instructions::new();
    for line in INPUT.lines() {
        let instruction = Instruction::new(line);
        instructions.push(instruction);
    }
    (state,instructions)
}

fn problem1(mut state: State,instructions: &Instructions) {
    for instruction in instructions {
        state.update(instruction);
    }
    state.rope_travel();

}

fn problem2(mut state: State,instructions: &Instructions) {
    
}