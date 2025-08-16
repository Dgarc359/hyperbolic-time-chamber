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
        let mut report = report.to_vec();
        for i in 0..report.len() {
            let removed_thing = report.remove(i);
            if is_report_safe(&report) { return true; }
            report.insert(i, removed_thing);
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
        number_rows.iter().filter(|x| is_report_safe(x)).count()
    );

    println!("Part 2 solution: {}", number_rows.iter().filter(|x| is_report_almost_safe(x)).count());
}
