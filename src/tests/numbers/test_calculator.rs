#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use crate::numbers::calculator::Calculator;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    // use super::super::calculator::Calculator;


    #[test]
    fn sum_test() {
        let calc = Calculator::new(2, 3);
        let sum = calc.sum();
        assert_eq!(sum, 5);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }
}