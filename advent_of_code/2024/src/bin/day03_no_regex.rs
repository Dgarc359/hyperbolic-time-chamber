use advent2024::read_stdin_to_string;

fn part1_solution(s: &str) -> u32 {
    calculate_solution(s, false)
}

fn part2_solution(s: &str) -> u32 {
    calculate_solution(s, true)
}

fn calculate_solution(s: &str, pay_attention_to_donts: bool) -> u32 {
    let mut rest = s;
    let mut solution = 0;
    let mut enabled = true;
    while !rest.is_empty() {
        if let Some(x) = rest.strip_prefix("do()") {
            // it's do()
            enabled = true;
            rest = x;
        } else if let Some(x) = rest.strip_prefix("don't()") {
            if pay_attention_to_donts {
                enabled = false;
            }
            rest = x;
        } else if let Some(split_rest) = rest.strip_prefix("mul(") {
            let Some((before_comma, after_comma)) = split_rest.split_once(',')
            else { break };
            let Some((before_paren, after_paren)) = after_comma.split_once(')')
            else { break };
            let (Ok(x), Ok(y))
            = (before_comma.parse::<u32>(), before_paren.parse::<u32>())
            else {
                rest = &rest[1..];
                continue;
            };
            if enabled {
                solution += x * y
            }
            rest = after_paren;
        } else {
            rest = &rest[1..];
        }
    }
    solution
}

fn main() {
    let input_string = read_stdin_to_string();
    println!("Part 1 solution: {}", part1_solution(&input_string));
    println!("Part 2 solution: {}", part2_solution(&input_string));
}