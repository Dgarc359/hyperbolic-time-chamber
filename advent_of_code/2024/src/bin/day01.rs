use std::collections::HashMap;

use advent2024::DupeCounterExt;

fn main() {
    let mut left_list: Vec<i32>;
    let mut right_list: Vec<i32>;

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

            (left_num, right_num)
        })
        .unzip();
    left_list.sort();
    right_list.sort();

    // This is what we want to eventually be able to do
    let right_appearances: HashMap<i32, i32> = right_list.iter().copied().count_dupes().collect();
    //let right_appearances: HashMap<i32, i32> =
    //    DupeCounter::new(right_list.iter().copied()).collect();

    let s = left_list
        .iter()
        .copied()
        .filter_map(|num| Some(num * right_appearances.get(&num).copied()?))
        .sum::<i32>();

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

   
    println!("Part 2 solution: {s}");
}
