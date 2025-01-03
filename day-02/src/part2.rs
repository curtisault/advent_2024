const MAX_UNSAFE_VIOLATIONS: i32 = 2;

#[derive(Debug, Clone)]
struct Report {
    list: Vec<i32>,
    violations: i32,
    list_type: LevelType,
}

#[derive(Debug, Clone)]
enum LevelType {
    Ascending,
    Descending,
    Error,
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

    let result = reports_list
        .iter()
        .map(|list| {
            dbg!(&list);
            let first_check = initial_check(list.clone());
            dbg!(&first_check);
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
        })
        .sum::<i32>();

    dbg!(result);
    Ok(result)
}

fn initial_check(list: Vec<i32>) -> Report {
    let mut init_check = Report {
        list: list.clone(),
        violations: 0,
        list_type: asc_or_desc_list(&list),
    };

    init_check = check_asc_or_desc_violations(init_check);

    if init_check.violations > 0 {
        return init_check;
    }

    match init_check.violations {
        0 | 1 => return linear_violations(init_check),
        _ => return init_check,
    }
}

fn check_asc_or_desc_violations(report: Report) -> Report {
    match report.list_type {
        LevelType::Ascending => {
            return asc_violations(report);
        }
        LevelType::Descending => {
            return desc_violations(report);
        }
        LevelType::Error => {
            return report;
        }
    }
}

fn second_check(report: Report) -> Report {
    let report = check_asc_or_desc_violations(report);

    if report.violations >= MAX_UNSAFE_VIOLATIONS {
        return report;
    }

    return linear_violations(report);
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

fn asc_or_desc_list(list: &Vec<i32>) -> LevelType {
    let first = list[0];
    let last = list[list.len() - 1];

    match (first, last) {
        (first, last) if first < last => LevelType::Ascending,
        (first, last) if first > last => LevelType::Descending,
        _ => LevelType::Error,
    }
}

fn asc_violations(report: Report) -> Report {
    if report.violations >= MAX_UNSAFE_VIOLATIONS {
        return report;
    }

    let asc_violation_index = asc_violation_index(&report.list);

    if asc_violation_index >= 0 {
        return remove_violation(report, asc_violation_index);
    }

    return report;
}

fn desc_violations(report: Report) -> Report {
    let desc_violation_index = desc_violation_index(&report.list);

    if desc_violation_index >= 0 {
        return remove_violation(report, desc_violation_index);
    }

    return report;
}

// returns the index of the first ascending violation in the list or -1 if there are no violations
fn asc_violation_index(list: &Vec<i32>) -> i32 {
    let list_len = list.len() - 1;
    for i in 1..list_len {
        if list[i - 1] < list[i] && list[i] > list[i + 1] {
            return i as i32;
        }
    }

    return -1;
}

// returns the index of the first descending violation in the list or -1 if there are no violations
fn desc_violation_index(list: &Vec<i32>) -> i32 {
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
        assert_eq!(1, process("7 6 4 2 1")?);
        assert_eq!(0, process("1 2 7 8 9")?);
        assert_eq!(0, process("9 7 6 2 1")?);
        assert_eq!(1, process("1 3 2 4 5")?);
        assert_eq!(1, process("8 6 4 4 1")?);
        assert_eq!(1, process("1 3 6 7 9")?);
        Ok(())
    }
}
