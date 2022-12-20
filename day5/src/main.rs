use std::collections::VecDeque;

const INPUT: &str = include_str!("input.txt");
type Cargo = Vec<VecDeque<char>>;
type Instruction = (usize,usize,usize);
type Instructions = Vec<Instruction>; //(Count,From,To)
const STRUCT_LEN: usize = 9;
const STRUCT_HEIGHT: usize = 8;

fn main() {
    problem1();
    problem2();
}

fn problem1() {
    let (mut cargo,instructions) = parse_input(INPUT,STRUCT_LEN,STRUCT_HEIGHT);
    print_cargo(&cargo);
    for instruction in instructions {
        println!("Moving {} From {} To {}",instruction.0,instruction.1,instruction.2);
        do_move(&mut cargo,instruction);
        print_cargo(&cargo);
    }
    for i in 0 .. STRUCT_LEN {
        match cargo[i].pop_front() {
            Some(character) => print!("{}",character),
            None => continue,
        };
    };
}

fn problem2() {
    let (mut cargo,instructions) = parse_input(INPUT,STRUCT_LEN,STRUCT_HEIGHT);
    print_cargo(&cargo);
    for instruction in instructions {
        println!("Moving {} From {} To {}",instruction.0,instruction.1,instruction.2);
        big_move(&mut cargo,instruction);
        print_cargo(&cargo);
    }
    for i in 0 .. STRUCT_LEN {
        match cargo[i].pop_front() {
            Some(character) => print!("{}",character),
            None => continue,
        };
    };
}

fn parse_input(input: &str, struct_len: usize,struct_height: usize) -> (Cargo,Instructions) {
    let mut test: Cargo = Vec::with_capacity(struct_len);
    for _ in 0..struct_len {
        test.push(VecDeque::with_capacity(struct_len));
    }
    let mut instructions: Instructions = Instructions::new();
    let mut lines = input.lines();
    for i in 0..input.lines().count() {
        let line = lines.next().unwrap();
        if i < struct_height {
            parse_struct(&mut test, line,struct_len);
        }
        if i > struct_height + 1 {
            parse_method(&mut instructions,line);
        }
    }
    (test,instructions)
}

fn parse_struct(cargo: &mut Cargo, line: &str, struct_len: usize) {
    let chars: Vec<char> = line.chars().collect();
    for index in 0..struct_len {
        let char_index = 1 + ( 4 * index);
        if chars[char_index] != ' ' {
            cargo[index].push_back(chars[char_index]);
        }
    }

}

fn print_cargo(cargo: &Cargo) {
    for i in 0..cargo.len() {
        print!("{} ",i);
        for j in (0..cargo[i].len()).rev() {
            print!("{}",cargo[i][j]);
        }
        println!("");
    }
}

fn parse_method(instructions: &mut Instructions, line: &str) {
    let array: Vec<&str> = line.split(' ').collect();
    instructions.push(
                        (array[1].parse::<usize>().unwrap(),
                        array[3].parse::<usize>().unwrap() -1,
                        array[5].parse::<usize>().unwrap() -1));
}

fn do_move(cargo: &mut Cargo, instruction: Instruction) {
    for _ in 0..instruction.0{
        let temp = cargo[instruction.1].pop_front();
        //println!("Moving {} From {} To {}",instruction.0,instruction.1,instruction.2);
        match temp {
            Some(inst) => cargo[instruction.2].push_front(inst),
            None => continue,
        };
    }
}

fn big_move(cargo: &mut Cargo, instruction: Instruction) {
    let mut buffer: Vec<char> = Vec::new();
    for _ in 0..instruction.0{
        buffer.push(cargo[instruction.1].pop_front().unwrap())
    }
    for _ in 0..instruction.0{
        cargo[instruction.2].push_front(buffer.pop().unwrap());
    }
}

#[test]
fn test_pmethod() {
    let mut instructions: Instructions = Instructions::new();
    let input = "move 1 from 2 to 3";
    parse_method(&mut instructions,input);
    assert!(instructions[0].0 == 1 && instructions[0].1 == 1 && instructions[0].2 == 2);
}

#[cfg(test)]
const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
#[cfg(test)]
const TEST_LEN: usize = 3;
#[cfg(test)]
const TEST_HEIGHT: usize = 3;

#[test]
fn test_domove() {
    let (mut cargo,instructions) = parse_input(TEST_INPUT,TEST_LEN,TEST_HEIGHT);
    for instruction in instructions {
        do_move(&mut cargo,instruction);
    };
    assert!(cargo[0].pop_front().unwrap() == 'C');
    assert!(cargo[1].pop_front().unwrap() == 'M');
    assert!(cargo[2].pop_front().unwrap() == 'Z');
}