const INPUT: &str = include_str!("input.txt");
fn main() {
    let mut count = 0;
    let mut x:Vec<i32> = Vec::new();
    for line in INPUT.split("\n").into_iter()
    {
        if line == "" 
        {
            x.push(count);
            count = 0;
            continue
        }
        count += line.parse::<i32>().unwrap();
        
    }
    x.sort();
    x.reverse();
    let total = x[0] + x[1] + x[2];
    println!("{:?}",total);
}

