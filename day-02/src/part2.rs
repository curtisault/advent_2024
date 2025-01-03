const MAX_UNSAFE_VIOLATIONS: i32 = 2;

#[derive(Debug, Clone)]
struct Report {
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
            let first_check = initial_check(list.clone());
            dbg!(&first_check.list);
            dbg!(&first_check.violations);
            match first_check.violations {
                0 => return 1,
                1 => {
                    let final_violations = second_check(first_check);
                    dbg!(&final_violations);
                    if final_violations.violations >= MAX_UNSAFE_VIOLATIONS {
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

fn initial_check(list: Vec<i32>) -> Report {
    let mut init_check = Report {
        list: list.clone(),
        violations: 0,
    };
    init_check = asc_violations(init_check);

    if init_check.violations > 0 {
        return init_check;
    }

    init_check = desc_violations(init_check);

    if init_check.violations > 0 {
        return init_check;
    }

    match init_check.violations {
        0 | 1 => return linear_violations(init_check),
        _ => return init_check,
    }
}

fn second_check(report: Report) -> Report {
    let reports = asc_violations(report);

    if reports.violations >= MAX_UNSAFE_VIOLATIONS {
        return reports;
    }

    linear_violations(reports)
}

fn linear_violations(report: Report) -> Report {
    if report.violations >= MAX_UNSAFE_VIOLATIONS {
        return report;
    }

    let violations_index = linear_violations_index(report.list.clone());
    if violations_index >= 0 {
        return remove_violation(report, violations_index);
    }

    return report;
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

    return -1;
}

fn asc_violations(report: Report) -> Report {
    if report.violations >= MAX_UNSAFE_VIOLATIONS {
        return report;
    }

    let asc_violation_index = asc_violation_index(report.list.clone());

    if asc_violation_index >= 0 {
        return remove_violation(report, asc_violation_index);
    }

    return report;
}

fn desc_violations(report: Report) -> Report {
    let desc_violation_index = desc_violation_index(report.list.clone());

    if desc_violation_index >= 0 {
        let report_with_removed_index = remove_violation(report, desc_violation_index);
        return report_with_removed_index;
    }

    return report;
}

// returns the index of the first ascending violation in the list or -1 if there are no violations
fn asc_violation_index(list: Vec<i32>) -> i32 {
    let list_len = list.len() - 1;
    for i in 1..list_len {
        if list[i - 1] < list[i] && list[i] > list[i + 1] {
            return i as i32;
        }
    }

    return -1;
}

// returns the index of the first descending violation in the list or -1 if there are no violations
fn desc_violation_index(list: Vec<i32>) -> i32 {
    let list_len = list.len() - 1;
    for i in 1..list_len {
        if list[i - 1] > list[i] && list[i] < list[i + 1] {
            return i as i32;
        }
    }

    return -1;
}

fn remove_violation(mut report: Report, index: i32) -> Report {
    report.list.remove(index as usize);
    report.violations += 1;
    report
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
