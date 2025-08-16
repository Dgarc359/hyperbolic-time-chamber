use advent2024::{SkipNthExt, ConsecutiveOverlappingPairsExt};

// some rules of thumb
// if you see &Vec<T>, replace with &[T]
// (because there is nothing you could do to a &Vec<T> you couldn't do to a &[T])
// if you see &String, replace with &str
// (because there is nothing you could do to a &String you couldn't do to a &str)

// yeetus?
// is as if you write:
// match yeetus {
//   Some(x) => x,
//   None => return None,
// }

fn is_report_safe(report: impl Iterator<Item=i32>) -> bool {
    // this is a problem
    // let first_pair_sign = (report[1]-report[0]).signum();

    let mut first_pair_sign:Option<i32> = None;
    report.consecutive_pairs().all(|(left_num, right_num)| {
        let pair_sign = (right_num-left_num).signum();
        let pair_delta = (right_num-left_num).abs();
        // Question 1: Is it increasing/decreasing in the same direction as the first pair?
        // Question 2: Is it changing by between 1 and 3 (inclusive)?

        let pair_sign_to_compair_sign = match first_pair_sign {
            Some(sign) => sign,
            None => pair_sign,
        };

        
        first_pair_sign = Some(pair_sign_to_compair_sign);

        (pair_sign_to_compair_sign == pair_sign) && (pair_delta >= 1 && pair_delta <= 3)
    })
}

fn is_report_almost_safe(report: &[i32]) -> bool {
    if is_report_safe(report.iter().copied()) {
        true
    } else {
        for i in 0..report.len() {
            // if is_report_safe(SkipNth::new(report.iter().copied(), i)) {
            //     return true;
            // }

            if is_report_safe(report.iter().copied().skip_nth(i)) {
                return true;
            }
        }
        false
    }
}
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
        number_rows.iter().filter(|x| is_report_safe(x.iter().copied())).count()
    );

    println!("Part 2 solution: {}", number_rows.iter().filter(|x| is_report_almost_safe(x)).count());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn skip_nth_test() {
        let skippable = vec![1,2,3,4,5];

        // let please_skip = SkipNth::new(skippable.into_iter(), 1);
        let please_skip = skippable.into_iter().skip_nth(1);

        assert_eq!(vec![1,3,4,5], please_skip.collect::<Vec<i32>>())
    }
    #[test]
    fn skip_0th_test() {
        let skippable = vec![1,2,3,4,5];

        let please_skip = skippable.into_iter().skip_nth(0);

        assert_eq!(vec![2,3,4,5], please_skip.collect::<Vec<i32>>())
    }
}