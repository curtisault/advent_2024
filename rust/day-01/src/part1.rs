#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut left_vec = vec![];
    let mut right_vec = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left_vec.push(items.next().unwrap().parse::<i32>().unwrap());
        right_vec.push(items.next().unwrap().parse::<i32>().unwrap());
    }

    left_vec.sort();
    right_vec.sort();

    let result: i32 = std::iter::zip(left_vec, right_vec)
        .map(|(left, right)| (left - right).abs())
        .sum();

    // dbg!(result);
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
                    4   3
                    2   5
                    1   3
                    3   9
                    3   3";

        assert_eq!("11", process(input)?);
        Ok(())
    }
}
