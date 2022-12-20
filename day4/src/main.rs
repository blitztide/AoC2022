const INPUT: &str = include_str!("input.txt");
const MAX_INPUT: usize = 100;
type Array = [bool;MAX_INPUT];

fn main() {
    let mut enclosing: i32 = 0;
    let mut overlapping: i32 = 0;
    let mut schedule_1: Array;
    let mut schedule_2: Array;
    for line in INPUT.lines() {
        (schedule_1,schedule_2) = get_array(line);
        enclosing += check_enclosing(schedule_1, schedule_2) as i32;
        overlapping += check_overlap(schedule_1,schedule_2) as i32;
    }
    println!("Number of enclosing jobs {}", enclosing);
    println!("Number of overlapping jobs {}", overlapping);
}

fn get_array(input: &str) -> (Array,Array) {
    let mut return_array: (Array,Array) = ([false;MAX_INPUT],[false;MAX_INPUT]);
    let schedules: Vec<&str> = input.split(',').collect();
    for i in 0..2{
        let x: Vec<&str> = schedules[i].split('-').collect();
        for j in x[0].parse::<usize>().unwrap() .. x[1].parse::<usize>().unwrap() + 1
         {
            match i {
                0 => return_array.0[j] = true,
                1 => return_array.1[j] = true,
                _ => panic!("Indexing is wrong?!")
            };
        }
    }
    return_array
}

fn check_enclosing(input_1: Array,input_2: Array) -> bool {
    let mut encloses1_array: Array = [false;MAX_INPUT];
    let mut encloses2_array: Array = [false;MAX_INPUT];
    for i in 0..input_1.len() {
        encloses1_array[i] = !input_1[i] || input_2[i];
        encloses2_array[i] = !input_2[i] || input_1[i];
    }
    let mut input_1_false = false;
    let mut input_2_false = false;
    for i in 0..input_1.len() {
        if encloses1_array[i] == false {
            input_1_false = true;
        }
        if encloses2_array[i] == false {
            input_2_false = true;
        }
    }
    !(input_1_false & input_2_false)
}

fn check_overlap(input_1: Array, input_2: Array) -> bool {
    for i in 0..input_1.len() {
        match input_1[i] & input_2[i] {
            true => return true,
            false => continue
        };
    }
    false
}

#[test]
fn test_array() {
    let input = "2-5,7-10";
    let mut test1: Array = [false;MAX_INPUT];
    let mut test2: Array = [false;MAX_INPUT];
    test1[2]=true;
    test1[3]=true;
    test1[4]=true;
    test1[5]=true;
    test2[7]=true;
    test2[8]=true;
    test2[9]=true;
    test2[10]=true;
    (test1,test2) = get_array(input);
    for y in 0 .. MAX_INPUT {
        assert!(test1[y] == test2[y]);
    }
}

#[test]
fn test_enclosing() {
    let (mut schedule_1, mut schedule_2) = get_array("4-5,6-9");
    assert!(check_enclosing(schedule_1, schedule_2) == false);
    (schedule_1,schedule_2) = get_array("4-5,2-10");
    assert!(check_enclosing(schedule_1, schedule_2) == true);
}

#[test]
fn test_overlap() {
    let (mut schedule_1, mut schedule_2) = get_array("1-3,3-6");
    assert!(check_overlap(schedule_1, schedule_2) == true);
    (schedule_1,schedule_2) = get_array("1-6,10-99");
    assert!(check_overlap(schedule_1,schedule_2) == false);
}