pub fn plus_one(num: &i32) -> i32 {
    num + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        assert_eq!(plus_one(&1), 2);
        assert_eq!(plus_one(&0), 1);
        assert_eq!(plus_one(&-1), 0);
    }
}
