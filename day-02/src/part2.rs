const MAX_UNSAFE_VIOLATIONS: usize = 2;

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
            dbg!(list.clone());
            // returns a tuple of the list and the violations indexes
            let violations = check_violations(list.clone(), vec![]);

            // dbg!(list.clone());
            dbg!(&violations);

            if violations.1.len() >= MAX_UNSAFE_VIOLATIONS {
                return 1;
            } else {
                return 0;
            }
        })
        .sum::<i32>();

    dbg!(result);
    Ok(6)
}

fn check_violations(mut list: Vec<i32>, mut violations: Vec<usize>) -> (Vec<i32>, Vec<usize>) {
    (list, violations) = asc_or_desc_violations(list.clone(), violations);

    match (list.clone(), violations.clone()) {
        (_l, v) if v.len() == 0 => linear_violations(list, violations),

        (_l, v) if v.len() >= MAX_UNSAFE_VIOLATIONS => (list, violations),
        (l, v) if v.len() == 1 => {
            dbg!(violations.clone());
            let index_to_remove = violations[0];
            let mut new_list = list.clone();
            dbg!(l.clone());
            new_list.remove(index_to_remove);
            dbg!(new_list.clone());

            (list, violations) = linear_violations(new_list.clone(), v.clone());
            if violations.len() >= MAX_UNSAFE_VIOLATIONS {
                return (list, violations);
            } else {
                check_violations(new_list, v)
            }
        }

        _ => (list, violations),
    }
}

fn linear_violations(list: Vec<i32>, mut violations: Vec<usize>) -> (Vec<i32>, Vec<usize>) {
    for i in 1..list.len() {
        let diff = (list[i] - list[i - 1]).abs();
        match diff {
            1 | 2 | 3 => continue,
            _ => violations.push(i + 1),
        };
    }

    (list, violations)
}

fn asc_or_desc_violations(list: Vec<i32>, mut violations: Vec<usize>) -> (Vec<i32>, Vec<usize>) {
    let list_length = list.len() - 1;

    for i in 1..list_length {
        if list[i - 1] < list[i] && list[i] > list[i + 1] {
            violations.push(i + 1);
        }

        if list[i - 1] > list[i] && list[i] < list[i + 1] {
            violations.push(i + 1);
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
