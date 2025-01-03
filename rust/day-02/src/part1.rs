#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i32> {
    let mut reports_list = vec![];

    for line in input.lines() {
        let reports_vec: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        reports_list.push(reports_vec);
    }

    // dbg!(reports_list.clone());

    let result = reports_list
        .iter()
        .map(|list| {
            let mut is_safe = all_increasing(list.clone()) || all_decreasing(list.clone());

            if !is_safe {
                return 0;
            }

            for i in 1..list.len() {
                let diff = (list[i] - list[i - 1]).abs();
                match diff {
                    1 | 2 | 3 => is_safe = true,
                    _ => is_safe = false,
                };

                if !is_safe {
                    break;
                }
            }

            if is_safe {
                return 1;
            }

            0
        })
        .sum::<i32>();

    Ok(result)
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
