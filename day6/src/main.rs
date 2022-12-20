use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    problem1(INPUT);
    problem2(INPUT);
}

fn problem1(input: &str) -> usize{
    let char_array: Vec<char> = input.chars().collect();
    println!("Length is: {}",char_array.len());
    for i in 0..char_array.len() {
        if check_duplicates(&char_array[i..i+4],4) {
            println!("Offset found at {}", i+4);
            return i+4;
        }
    }
    return 0;
}

fn problem2(input: &str) -> usize{
    let char_array: Vec<char> = input.chars().collect();
    println!("Length is: {}",char_array.len());
    for i in 0..char_array.len() {
        if check_duplicates(&char_array[i..i+14],14) {
            println!("Offset found at {}", i+14);
            return i+14;
        }
    }
    return 0;
}

fn check_duplicates(input: &[char],size: usize) -> bool {
    let mut hashmap: HashMap<String,i32> = HashMap::new();
    for i in 0..size {
        if hashmap.contains_key(&input[i].to_string()) {
            return false;
        } else {
            hashmap.insert(input[i].to_string(),1);
        }
    }
    true
}


#[test]
fn test() {
    let test1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert!(problem1(test1) == 7);
    let test2 = "nppdvjthqldpwncqszvftbrmjlhg";
    assert!(problem1(test2) == 6);
    let test3 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert!(problem1(test3) == 10);
    let test4 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert!(problem1(test4) == 11)
}