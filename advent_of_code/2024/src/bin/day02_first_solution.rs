// some rules of thumb
// if you see &Vec<T>, replace with &[T]
// (because there is nothing you could do to a &Vec<T> you couldn't do to a &[T])
// if you see &String, replace with &str
// (because there is nothing you could do to a &String you couldn't do to a &str)

fn is_report_safe(report: &[i32]) -> bool {
    let first_pair_sign = (report[1]-report[0]).signum();
    report.windows(2).all(|pair| {
        let left_num = pair[0];
        let right_num = pair[1];
        let pair_sign = (right_num-left_num).signum();
        let pair_delta = (right_num-left_num).abs();
        // Question 1: Is it increasing/decreasing in the same direction as the first pair?
        // Question 2: Is it changing by between 1 and 3 (inclusive)?

        (first_pair_sign == pair_sign) && (pair_delta >= 1 && pair_delta <= 3)
    })
}

fn is_report_almost_safe(report: &[i32]) -> bool {
    if is_report_safe(report) {
        true
    } else {
        report.iter().enumerate().any(|(index, _item)| {
            let mut report_without_index_value: Vec<i32> = report.to_vec();
            report_without_index_value.remove(index);

            is_report_safe(&report_without_index_value)
        })
    }
}

//enum LevelDelta {
//    Increasing,
//    Decreasing,
//}
//fn is_report_safe_old(report: &Vec<i32>) -> bool {
//    let mut level_delta: Option<LevelDelta> = None;
//    let mut previous_number: Option<i32> = None;

//    report
//        .iter()
//        .map(|n| {
//            let num = n.clone();

//            // if previous_number.is_none() {
//            //     previous_number = Some(num)
//            // }

//            let current_level_delta =
//                num.saturating_sub(previous_number.unwrap());

//            match level_delta {
//                Some(LevelDelta::Increasing) => {
//                    // previous number must be lower than current number
//                    if current_level_delta < 1 || current_level_delta > 3 {
//                        Some(false)
//                    } else {
//                        Some(true)
//                    }
//                }
//                Some(LevelDelta::Decreasing) => {
//                    // previous number must be lower than current number
//                    if current_level_delta > -1 || current_level_delta < -3 {
//                        Some(false)
//                    } else {
//                        Some(true)
//                    }
//                }
//                None => {
//                    // we haven't set a level delta yet... we can just set current level delta
//                    if current_level_delta < 0 {
//                        level_delta = Some(LevelDelta::Decreasing);
//                    } else if current_level_delta > 0 {
//                        level_delta = Some(LevelDelta::Increasing);
//                    }
//                    None
//                }
//            }
//        })
//        .filter_map(|x| x)
//        .all(|rating| rating)
//}

fn main() {
    let number_rows: Vec<Vec<i32>> = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let numbers: Vec<&str> = line.split(" ").collect();
            let nums: Vec<i32> = numbers
                .iter()
                .copied()
                .map(|num| {
                    num.parse::<i32>().expect("WE COULDNT GET A NUMBER AHHH")
                })
                .collect();

            nums
        })
        .collect();

    println!(
        "Part 1 solution: {}",
        number_rows.iter().filter(|x| is_report_safe(x)).count()
    );

    println!("Part 2 solution: {}", number_rows.iter().filter(|x| is_report_almost_safe(x)).count());
}
