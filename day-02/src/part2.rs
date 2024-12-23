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
            let unsafe_violations = ascending_or_descending_violations(list.clone());
            let mut unsafe_count = 0;

            for i in 1..list.len() {
                let diff = (list[i] - list[i - 1]).abs();
                match diff {
                    1 | 2 | 3 => continue,
                    _ => unsafe_count += 1,
                };

                if unsafe_count >= unsafe_violations {
                    return 0;
                }
            }

            1
        })
        .sum::<i32>();

    Ok(result)
}

fn ascending_or_descending_violations(list: Vec<i32>) -> i32 {
    let mut violations = 0;
    let list_length = list.len() - 1;

    for i in 1..list_length {
        if list[i - 1] < list[i] && list[i] > list[i + 1] {
            violations += 1;
        }

        if list[i - 1] > list[i] && list[i] < list[i + 1] {
            violations += 1;
        }
    }

    dbg!(list.clone());
    dbg!(violations);
    violations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!(
            4,
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
