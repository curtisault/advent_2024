const MAX_UNSAFE_VIOLATIONS: i32 = 2;

struct Reports {
    list: Vec<i32>,
    violations: i32,
}

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
            // dbg!(list.clone());
            // returns a tuple of the list and the violations indexes
            // let violations = check_violations(list.clone(), vec![]);

            dbg!(&list);
            let initial_violations = initial_check(list.clone(), 0);
            dbg!(&initial_violations.list);
            dbg!(&initial_violations.violations);
            match initial_violations.violations {
                0 => return 1,
                1 => {
                    let violations =
                        second_check(initial_violations.list, initial_violations.violations);
                    dbg!(&violations.list);
                    dbg!(&violations.violations);
                    if violations.violations >= MAX_UNSAFE_VIOLATIONS {
                        return 0;
                    } else {
                        return 1;
                    }
                }
                _ => return 0,
            }
            // dbg!(list.clone());
            // dbg!(&violations);
        })
        .sum::<i32>();

    dbg!(result);
    Ok(result)
}

fn initial_check(list: Vec<i32>, violations: i32) -> Reports {
    let asc_desc_violations = asc_or_desc_violations(list.clone(), violations);

    match asc_desc_violations.violations {
        0 | 1 => return linear_violations(list, violations),
        _ => return Reports { list, violations },
    }
}

fn second_check(list: Vec<i32>, violations: i32) -> Reports {
    let reports = asc_or_desc_violations(list.clone(), violations);

    if reports.violations >= MAX_UNSAFE_VIOLATIONS {
        return Reports { list, violations };
    } else {
        linear_violations(reports.list, reports.violations);
    }

    return Reports { list, violations };
}

fn linear_violations(mut list: Vec<i32>, mut violations: i32) -> Reports {
    if violations >= MAX_UNSAFE_VIOLATIONS {
        return Reports { list, violations };
    }

    let violations_index = linear_violations_index(list.clone());
    if violations_index != 0 {
        list = remove_violation(list, violations_index);
        violations += 1;
    }

    return Reports { list, violations };
}

fn linear_violations_index(list: Vec<i32>) -> i32 {
    for i in 1..list.len() {
        let diff = (list[i] - list[i - 1]).abs();
        match diff {
            1 | 2 | 3 => continue,
            _ => {
                return i as i32;
            }
        };
    }

    return 0;
}

fn asc_or_desc_violations(mut list: Vec<i32>, mut violations: i32) -> Reports {
    if violations >= MAX_UNSAFE_VIOLATIONS {
        return Reports { list, violations };
    }

    let asc_violation_index = asc_violation_index(list.clone());

    if asc_violation_index != 0 {
        list = remove_violation(list, asc_violation_index);
        violations += 1;
    }

    let desc_violation_index = desc_violation_index(list.clone());

    if desc_violation_index != 0 {
        list = remove_violation(list, desc_violation_index);
        violations += 1;
    }

    return Reports { list, violations };
}

fn asc_violation_index(list: Vec<i32>) -> i32 {
    let list_len = list.len() - 1;
    for i in 1..list_len {
        if list[i - 1] < list[i] && list[i] > list[i + 1] {
            return i as i32;
        }
    }

    return 0;
}

fn desc_violation_index(list: Vec<i32>) -> i32 {
    let list_len = list.len() - 1;
    for i in 1..list_len {
        if list[i - 1] > list[i] && list[i] < list[i + 1] {
            return i as i32;
        }
    }

    return 0;
}

fn remove_violation(list: Vec<i32>, index: i32) -> Vec<i32> {
    let mut new_list = list;
    new_list.remove(index as usize);
    return new_list;
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
