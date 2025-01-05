mod paths;

pub use paths::*;

#[cfg(test)]
mod tests {

    use crate::GET_ID_CAP26_LOCAL;

    #[test]
    fn test_asciisum() {
        let ascii_sum = |s: &str| s.chars().fold(0, |acc, c| acc + c as u32);
        assert_eq!(ascii_sum("GETID"), GET_ID_CAP26_LOCAL as u32);
    }
}
