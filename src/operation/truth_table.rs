use super::rpn::rpn;

pub fn print_truth_table(formula: &str) {
    if let Some(_) = formula.find(|c: char| !"!&|^>=".contains(c) && !c.is_ascii_uppercase()) {
        panic!("Proposition contains incorrect characters!");
    }

    let letters = formula
        .chars()
        .filter(|c| c.is_ascii_uppercase());

    //TODO: find the best way to compute all possibilities for a set of letters
}