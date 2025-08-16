use advent2024::read_stdin_to_string;
use regex::Regex;

// remember this rule?:
// when you write &String, replace it with &str

fn part1_solution(s: &str) -> u32 {
    // mul\((\d+),(\d+)\)
    // this should just get mul(<x>,<y>) matches in the string
    // unwrapping here is ok bcuz if it fails it will fail every time and also our code depends on this working wtf bro!
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(s).map(|captures|{
        let x: u32 = captures[1].parse().unwrap();
        let y: u32 = captures[2].parse().unwrap();

        x * y
    }).sum::<u32>()
}

fn part2_solution(s: &str) -> u32 {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut enabled = true;
    re.captures_iter(s).map(|captures|{
        if captures[0].starts_with("mul") {
            let x: u32 = captures[1].parse().unwrap();
            let y: u32 = captures[2].parse().unwrap();
            if enabled {
                x * y
            } else {
                0
            }
        } else if &captures[0] == "don't()" {
            enabled = false;
            0
        } else {
            assert_eq!(&captures[0], "do()");
            enabled = true;
            0
        }
    }).sum::<u32>()
}

fn main() {
    let input_string = read_stdin_to_string();
    println!("Part 1 solution: {}", part1_solution(&input_string));
    println!("Part 2 solution: {}", part2_solution(&input_string));
}