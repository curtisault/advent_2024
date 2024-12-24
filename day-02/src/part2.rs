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
            // returns a tuple of the list and the violations indexes
            let mut violations = ascending_or_descending_violations(list.clone(), vec![]);

            dbg!(list.clone());
            dbg!(violations);

        })
        .sum::<i32>();

    dbg!(result);
    Ok(6)
}

fn check_violations(list: Vec<i32>, mut violations: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let max_unsafe_violations = 2;
    match (list, violations) {
        (list, violations) => {
            if violations.len() >= max_unsafe_violations {
                return 0;
            }
            else if violations.len() > 0 {
                index_to_remove = violations[0];
                let new_list = list.clone().remove(index_to_remove);
            }
        }
    }

}

fn linear_violations(list: Vec<i32>) -> (Vec<i32>,Vec<i32>) {
    let mut violations = vec![];

    for i in 1..list.len() {
        let diff = (list[i] - list[i - 1]).abs();
        match diff {
            1 | 2 | 3 => continue,
            _ => violations.push(list[i]),
        };
    }

    (list, violations)
}

fn ascending_or_descending_violations(list: Vec<i32>, mut violations: Vec<i32>) -> (Vec<i32>, Vec<i32> {
    let list_length = list.len() - 1;

    for i in 1..list_length {
        if list[i - 1] < list[i] && list[i] > list[i + 1] {
            violations.push(list[i]);
        }

        if list[i - 1] > list[i] && list[i] < list[i + 1] {
            violations.push(list[i]);
        }
    }

    (list, violations)
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
