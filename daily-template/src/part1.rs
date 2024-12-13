#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    todo!("Write code");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("", process("")?);
        Ok(())
    }
}
