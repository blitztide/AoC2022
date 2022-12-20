const INPUT: &str = include_str!("input.txt");
fn main() {
    
    let mut score = 0;

    for line in INPUT.lines()
    {
        // A Rock B Paper C Scissors
        // X Rock Y Paper Z Scissors
        // X Lose Y Draw Z Win
        // Score 0 Lose 3 Draw 6 Win
        let lose = 0;
        let draw = 3;
        let win = 6;
        let rock = 1;
        let paper = 2;
        let scissors = 3;
        match line {
            "A X" => score += lose + scissors,
            "A Y" => score += draw + rock,
            "A Z" => score += win + paper,
            "B X" => score += lose + rock,
            "B Y" => score += draw + paper,
            "B Z" => score += win + scissors,
            "C X" => score += lose + paper,
            "C Y" => score += draw + scissors,
            "C Z" => score += win + rock,
            _ => continue
        }

            
    }
    println!("{}",score); 
}