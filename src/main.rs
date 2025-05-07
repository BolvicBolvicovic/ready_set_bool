mod operation;

use operation::{
    adder::adder,
    grey_code::grey_code,
    multiplier::multiplier,
    rpn::rpn,
    truth_table::print_truth_table,
    negation_normal_form::negation_normal_form,
};

fn main() {
    assert_eq!(adder(5, 6), 11);
    assert_eq!(multiplier(5, 6), 30);
    assert_eq!(grey_code(14), 9);
    assert_eq!(rpn("1011||="), true);
    assert_eq!(rpn("1011||=0>"), false);
    print_truth_table("AB&C|");
    assert_eq!(negation_normal_form("A!!"), "A");
    assert_eq!(negation_normal_form("AB>"), "A!B|");
    assert_eq!(negation_normal_form("ABAA|>=B|"), "AB!AA||=B|");
}
