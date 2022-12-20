const INPUT: &str = include_str!("input.txt");


fn main() {
    challenge_1();
    challenge_2();
}

fn challenge_2() {
    let mut priorities:Vec<usize> = Vec::new();
    let mut group_count = 0;
    let mut group_array: [i32;53] = [0;53];

    for line in INPUT.lines(){
        group_count += 1;
        for character in line.chars() {
            group_array[get_score(character) as usize] |=  1 << group_count;
        }

        if group_count == 3 {
            group_count = 0;
            println!("{:?}",group_array);
            priorities.push(group_array.iter().position(|&x| x==14).unwrap());
            for i in 1..53 {
                group_array[i] = 0;
            }
        }
    }
    println!("{:?}",priorities.iter().sum::<usize>());
}

fn challenge_1() {
    let mut priorities:Vec<char> = Vec::new();
    for line in INPUT.lines(){
        let (half1,half2) = line.split_at(line.len()/2);
        for char in half1.chars() {
            if half2.contains(char) {
                priorities.push(char);
                break;
            }
        }
    }
    let mut score = 0;
    println!("{:?} {}",priorities, priorities.len());
    for priority in priorities{
        // z = 122 Z=90
        score += get_score(priority);
    }
    println!("{}",score);
}

fn get_score(input:char) -> u32 {
    if input as u32 >= 96 {
        input as u32 - 96
    } else {
        input as u32 - 64 + 26
    }
}

#[test]
fn check_score(){
    println!("{}",'A' as i32);
    for char in 'a'..'z'{
        println!("{} {}",char, get_score(char));
    }
    for char in 'A'..'Z'{
        println!("{} {}",char, get_score(char));
    }
}