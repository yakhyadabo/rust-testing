
#[cfg(test)]
mod tests {
    use crate::numbers::calculator::Calculator;
    use crate::strings::greetings::Greetings;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    // use super::super::calculator::Calculator;


    #[test]
    fn sum_test() {
        let greetings = Greetings::new("Max".to_string());
        let hello = greetings.hello();
        assert_eq!(hello, "Hello there Max");
    }

}