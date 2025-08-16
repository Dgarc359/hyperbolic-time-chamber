use std::collections::HashMap;

fn main() {
    let mut left_list: Vec<i32>;
    let mut right_list: Vec<i32>;
    let mut right_appearances: HashMap<i32, i32> = HashMap::new();

    (left_list, right_list) = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let num_str_arr = line.split_once("   ").unwrap();

            let left_num: i32 = num_str_arr
                .0
                .parse()
                .expect("WE DIDNT GET A NUMBER AHHHHHHHHH");
            let right_num: i32 = num_str_arr
                .1
                .parse()
                .expect("WE DIDNT GET A NUMBER AHHHHHHHHH");

            right_appearances.insert(
                right_num,
                right_appearances.get(&right_num).copied().unwrap_or(0) + 1,
            );

            (left_num, right_num)
        })
        .unzip();
    left_list.sort();
    right_list.sort();

    assert_eq!(left_list.len(), right_list.len());

    println!(
        "Part 1 solution: {}",
        left_list
            .iter()
            .copied()
            .zip(right_list.iter().copied())
            .map(|(left_num, right_num)| (left_num - right_num).abs())
            .sum::<i32>()
    );

    let one_line_part_two = left_list
        .iter()
        .copied()
        .map(|left_num| {
            right_appearances.get(&left_num).copied().unwrap_or(0) * left_num
        })
        .sum::<i32>();
    println!("Part 2 solution: {one_line_part_two}");
}
