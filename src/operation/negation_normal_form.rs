use super::rpn::rpn_format;

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
            let ope_to_jump = var_count - ope_count - 2;
            let mut sub_to_chars = substring.chars();
            let mut ope_jumped = 0;
            let mut potential_char_idx = 0;
            let mut current_idx = 0;
            while ope_jumped < ope_to_jump {
                if "&|^=".contains((&mut sub_to_chars).nth(0).unwrap()) {
                    ope_jumped += 1;
                    potential_char_idx -= 3;
                } else {
                    potential_char_idx += 1;
                }
                current_idx += 1;
            }
            if potential_char_idx > 0 {
                potential_char_idx
            } else {
                current_idx
            }
        }
    }

    fn material_conditions(mut self) -> Self {

        while self.formula.contains('>') {
            let then = self.formula.find('>').unwrap();
            let left = self.formula[..then].to_string();
            let exclamation_idx = Self::a_index(&left) + 1;
            self.formula.remove(then);
            self.formula.insert(then, '|');
            self.formula.insert(exclamation_idx, '!');
        }
        self
    }
}

pub fn negation_normal_form(formula: &str) -> String {
    NNF::new(formula)
        .delete_double_negation()
        .material_conditions()
        .formula
}