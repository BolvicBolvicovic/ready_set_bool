mod operation;

use operation::{
    adder::adder,
    grey_code::grey_code,
    multiplier::multiplier,
    rpn::rpn,
    truth_table::print_truth_table,
};

fn main() {
    assert_eq!(adder(5, 6), 11);
    assert_eq!(multiplier(5, 6), 30);
    assert_eq!(grey_code(14), 9);
    assert_eq!(rpn("1011||="), true);
    assert_eq!(rpn("1011||=0>"), false);
    print_truth_table("formula");
}
