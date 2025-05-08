use super::rpn::rpn_format;
use crate::utils::swap_char::swap_char;

struct NNF {
    formula: String,
}

impl NNF {
    fn new(formula: &str) -> Self {
        rpn_format(formula);
        NNF {
            formula: formula.to_string(),
        }
    }

    fn delete_double_negation(mut self) -> Self {
        while self.formula.contains("!!") {
            self.formula = self.formula.replace("!!", "");
        }
        self
    }

    fn a_index(substring: &str) -> usize {        
        let ope_count = substring
            .chars()
            .into_iter()
            .filter(|&c| "&=|^".contains(c))
            .count();
        let var_count = substring
            .chars()
            .into_iter()
            .filter(|&c| c.is_ascii_uppercase())
            .count();

        if var_count - ope_count <= 1 {
            substring.rfind(['&', '|', '^', '=']).unwrap()
        } else {
            let mut current_index = 0;
            let mut a = 0;
            let mut b = 0;
            for c in substring.chars() {
                if "&|^=".contains(c) {
                    a -= 1;
                    b = current_index;
                } else if c.is_ascii_uppercase() {
                    a = b;
                    b = current_index;
                }
                current_index += 1;
            }
            a
        }
    }

    fn material_conditions(mut self) -> Self {

        while self.formula.contains('>') {
            let then = self.formula.find('>').unwrap();
            let left = self.formula[..then].to_string();
            let exclamation_idx = Self::a_index(&left) + 1;
            swap_char(&mut self.formula, then, '|').unwrap();
            self.formula.insert(exclamation_idx, '!');
        }
        self
    }
}

pub fn negation_normal_form(formula: &str) -> String {
    NNF::new(formula)
        .delete_double_negation()
        .material_conditions()
        .delete_double_negation()
        .formula
}