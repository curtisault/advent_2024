#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i32> {
    let mut reports_list = vec![];
    let mut safe_report_count = 0;

    for line in input.lines() {
        let reports_vec: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        reports_list.push(reports_vec);
    }

    for list in reports_list {
        let mut is_safe = all_increasing(list.clone()) || all_decreasing(list.clone());

        if !is_safe {
            continue;
        }

        // println!("{:?}", list);
        for i in 1..list.len() {
            let diff = (list[i] - list[i - 1]).abs();
            // println!("difference: {}", diff);
            match diff {
                1 | 2 | 3 => is_safe = true,
                _ => is_safe = false,
            };

            if !is_safe {
                break;
            }
        }

        // println!("is_safe: {}", is_safe);

        if is_safe {
            safe_report_count += 1;
        }
    }

    Ok(safe_report_count)
}

fn all_increasing(list: Vec<i32>) -> bool {
    for i in 1..list.len() {
        if list[i - 1] < list[i] {
            return false;
        }
    }

    true
}

fn all_decreasing(list: Vec<i32>) -> bool {
    for i in 1..list.len() {
        if list[i - 1] > list[i] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!(
            2,
            process(
            "7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9"
            )?
        );
        Ok(())
    }
}
