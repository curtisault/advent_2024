#[tracing::instrument]
pub fn process(_input: &str) -> String {
    "part1".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        todo!("Write tests");
        assert_eq!(process(""), "part1");
    }
}
