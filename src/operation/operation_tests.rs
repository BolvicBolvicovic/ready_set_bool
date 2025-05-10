#[cfg(test)]
mod operator_tests {
    use crate::operation::{
        multiplier::multiplier,
        adder::adder,
        grey_code::grey_code,
        rpn::rpn
    };

    #[test]
    fn test_adder() {
        assert_eq!(adder(5, 6), 11);
    }

    #[test]
    fn test_multiplier() {
        assert_eq!(multiplier(5, 6), 30);
    }

    #[test]
    fn test_grey_code() {
        assert_eq!(grey_code(14), 9);
    }

    #[test]
    fn test_rpn() {
        assert_eq!(rpn("1011||="), true);
        assert_eq!(rpn("1011||=0>"), false);
    }
}