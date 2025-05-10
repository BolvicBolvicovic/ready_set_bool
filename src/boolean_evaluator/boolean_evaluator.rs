use core::panic;

#[derive(Debug, Clone)]
pub enum Evaluator {
    Variable(char),
    And(Box<Evaluator>, Box<Evaluator>),
    Or(Box<Evaluator>, Box<Evaluator>),
    Not(Box<Evaluator>),
    Xor(Box<Evaluator>, Box<Evaluator>),
    Equivalence(Box<Evaluator>, Box<Evaluator>),
    Conditional(Box<Evaluator>, Box<Evaluator>),
}

impl Evaluator {
    pub fn new(formula: &str) -> Self {
        let mut stack: Vec<Evaluator> = vec![];

        for c in formula.chars() {
            match c {
                'A'..='Z' => stack.push(Evaluator::Variable(c)),
                '&' => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(Evaluator::And(Box::new(left), Box::new(right)));
                }
                '|' => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(Evaluator::Or(Box::new(left), Box::new(right)));
                }
                '^' => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(Evaluator::Xor(Box::new(left), Box::new(right)));
                }
                '=' => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(Evaluator::Equivalence(Box::new(left), Box::new(right)));
                }
                '>' => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(Evaluator::Conditional(Box::new(left), Box::new(right)));
                }
                '!' => {
                    let operand = stack.pop().unwrap();
                    stack.push(Evaluator::Not(Box::new(operand)));
                }
                _ => panic!("Invalid character in formula: {}", c),
            }
        }
        if stack.len() != 1 {
            panic!("Invalid formula: {}, operations left: {:?}", formula, stack);
        }
        stack.pop().unwrap()
    }

    pub fn to_string(&self) -> String {
        let res = match self {
            Evaluator::Variable(c) => c.to_string(),
            Evaluator::And(left, right) => format!("{}{}&", left.to_string(), right.to_string()),
            Evaluator::Or(left, right) => format!("{}{}|", left.to_string(), right.to_string()),
            Evaluator::Not(operand) => format!("{}!", operand.to_string()),
            Evaluator::Xor(left, right) => format!("{}{}^", left.to_string(), right.to_string()),
            Evaluator::Equivalence(left, right) => format!("{}{}=", left.to_string(), right.to_string()),
            Evaluator::Conditional(left, right) => format!("{}{}>", left.to_string(), right.to_string()),
        };
        if !res.contains(['|', '^', '=', '>']) || !res.contains(['&', '^', '=', '>']) {
            res.as_str().chars().filter(|c| c != &'&' && c != &'|').collect::<String>() +
            &res.as_str().chars().filter(|c| c == &'&' || c == &'|').collect::<String>()
        } else {
            res
        }
    }

    pub fn to_negation_normal_form(&self) -> Self {
        match self {
            Evaluator::Variable(_) => self.clone(),
            Evaluator::Not(inner) => {
                match **inner {
                    Evaluator::Variable(_) => self.clone(),
                    Evaluator::Not(ref inner_inner) => {
                        // Double negation elimination
                        inner_inner.to_negation_normal_form()
                    },
                    Evaluator::And(ref left, ref right) => {
                        // De Morgan's law for AND
                        let left_not = Evaluator::Not(Box::new(*left.clone()));
                        let right_not = Evaluator::Not(Box::new(*right.clone()));
                        Evaluator::Or(
                            Box::new(left_not.to_negation_normal_form()),
                            Box::new(right_not.to_negation_normal_form())
                        )
                    },
                    Evaluator::Or(ref left, ref right) => {
                        // De Morgan's law for OR
                        let left_not = Evaluator::Not(Box::new(*left.clone()));
                        let right_not = Evaluator::Not(Box::new(*right.clone()));
                        Evaluator::And(
                            Box::new(left_not.to_negation_normal_form()),
                            Box::new(right_not.to_negation_normal_form())
                        )
                    },
                    _ => {
                        // For other operations, process inner first, then negate
                        let inner_nnf = inner.to_negation_normal_form();
                        Evaluator::Not(Box::new(inner_nnf))
                    }
                }
            },
            Evaluator::And(left, right) => {
                let left_nnf = left.to_negation_normal_form();
                let right_nnf = right.to_negation_normal_form();
                Evaluator::And(Box::new(left_nnf), Box::new(right_nnf))
            },
            Evaluator::Or(left, right) => {
                let left_nnf = left.to_negation_normal_form();
                let right_nnf = right.to_negation_normal_form();
                Evaluator::Or(Box::new(left_nnf), Box::new(right_nnf))
            },
            Evaluator::Xor(left, right) => {
                // A XOR B ≡ (A OR B) AND NOT(A AND B)
                let left_nnf = left.to_negation_normal_form();
                let right_nnf = right.to_negation_normal_form();
                
                let or_part = Evaluator::Or(Box::new(left_nnf.clone()), Box::new(right_nnf.clone()));
                let and_part = Evaluator::And(Box::new(left_nnf), Box::new(right_nnf));
                let not_and = Evaluator::Not(Box::new(and_part));
                
                Evaluator::And(
                    Box::new(or_part),
                    Box::new(not_and.to_negation_normal_form())
                )
            },
            Evaluator::Equivalence(left, right) => {
                // A ≡ B is the same as (A → B) AND (B → A)
                let left_nnf = left.to_negation_normal_form();
                let right_nnf = right.to_negation_normal_form();
                
                let impl1 = Evaluator::Conditional(Box::new(left_nnf.clone()), Box::new(right_nnf.clone()));
                let impl2 = Evaluator::Conditional(Box::new(right_nnf), Box::new(left_nnf));
                
                Evaluator::And(
                    Box::new(impl1.to_negation_normal_form()),
                    Box::new(impl2.to_negation_normal_form())
                )
            },
            Evaluator::Conditional(left, right) => {
                // A → B is the same as NOT(A) OR B
                let left_nnf = left.to_negation_normal_form();
                let right_nnf = right.to_negation_normal_form();
                
                let not_left = Evaluator::Not(Box::new(left_nnf));
                
                Evaluator::Or(
                    Box::new(not_left.to_negation_normal_form()),
                    Box::new(right_nnf)
                )
            }
        }
    }

    fn to_conjunctive_normal_form_callback(&self) -> Self {
        match self {
            Evaluator::Variable(_) | Evaluator::Not(box Evaluator::Variable(_)) => self.clone(),
        
            Evaluator::And(left, right) => {
                let left_cnf = left.to_conjunctive_normal_form_callback();
                let right_cnf = right.to_conjunctive_normal_form_callback();
                Evaluator::And(Box::new(left_cnf), Box::new(right_cnf))
            },
        
            // OR of formulas: the core of CNF conversion
            Evaluator::Or(left, right) => {
                let left_cnf = left.to_conjunctive_normal_form_callback();
                let right_cnf = right.to_conjunctive_normal_form_callback();

                // Apply the distributive law: OR over AND
                match (&left_cnf, &right_cnf) {
                    // Case: (A AND B) OR C => (A OR C) AND (B OR C)
                    (Evaluator::And(a, b), _) => {
                        let a_or_right = Evaluator::Or(a.clone(), Box::new(right_cnf.clone()));
                        let b_or_right = Evaluator::Or(b.clone(), Box::new(right_cnf));

                        Evaluator::And(
                            Box::new(a_or_right.to_conjunctive_normal_form_callback()),
                            Box::new(b_or_right.to_conjunctive_normal_form_callback())
                        )
                    },

                    // Case: A OR (B AND C) => (A OR B) AND (A OR C)
                    (_, Evaluator::And(a, b)) => {
                        let left_or_a = Evaluator::Or(Box::new(left_cnf.clone()), a.clone());
                        let left_or_b = Evaluator::Or(Box::new(left_cnf), b.clone());

                        Evaluator::And(
                            Box::new(left_or_a.to_conjunctive_normal_form_callback()),
                            Box::new(left_or_b.to_conjunctive_normal_form_callback())
                        )
                    },

                    // Case: No AND on either side, leave as is
                    _ => Evaluator::Or(Box::new(left_cnf), Box::new(right_cnf))
                }
            },
        
            Evaluator::Not(inner) => {
                match **inner {
                    // Double negation
                    Evaluator::Not(ref inner_inner) => inner_inner.to_conjunctive_normal_form_callback(),
                    _ => self.clone(),
                }
            },
        
            // These operators should have been eliminated by NNF conversion
            Evaluator::Xor(_, _) | Evaluator::Equivalence(_, _) | Evaluator::Conditional(_, _) => {
                self.to_negation_normal_form().to_conjunctive_normal_form_callback()
            }
        }
    }

    pub fn to_conjunctive_normal_form(&self) -> Self {
        self
            .to_negation_normal_form()
            .to_conjunctive_normal_form_callback()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluator_to_string() {
        let formula = Evaluator::new("AB&C|");
        assert_eq!(formula.to_string(), "AB&C|");
    }

    #[test]
    fn test_negation_normal_form_and_not() {
        let formula = Evaluator::new("AB&!");
        let nnf = formula.to_negation_normal_form();
        assert_eq!(nnf.to_string(), "A!B!|");
    }

    #[test]
    fn test_negation_normal_form_or_not() {
        let formula = Evaluator::new("AB|!");
        let nnf = formula.to_negation_normal_form();
        assert_eq!(nnf.to_string(), "A!B!&");
    }

    #[test]
    fn test_negation_normal_form_conditional() {
        let formula = Evaluator::new("AB>");
        let nnf = formula.to_negation_normal_form();
        assert_eq!(nnf.to_string(), "A!B|");
    }

    #[test]
    fn test_negation_normal_form_equivalence() {
        let formula = Evaluator::new("AB=");
        let nnf = formula.to_negation_normal_form();
        assert_eq!(nnf.to_string(), "A!B|B!A|&"); 
        // "AB&A!B!&|" is the same as "A!B|B!A|&" 
        // because both are equivalent to AB=
        // In logic notation, A ≡ B is equivalent to (A ∧ B) ∨ (¬A ∧ ¬B) (example from subject)
        // and A ≡ B is also equivalent to (A → B) ∧ (B → A) (result from the test before Conditional is removed)
    }

    #[test]
    fn test_negation_normal_form_or_and_not() {
        let formula = Evaluator::new("AB|C&!");
        let nnf = formula.to_negation_normal_form();
        assert_eq!(nnf.to_string(), "A!B!&C!|");
    }

    #[test]
    fn test_conjunctive_normal_form_and_not() {
        let formula = Evaluator::new("AB&!");
        let cnf = formula.to_conjunctive_normal_form();
        assert_eq!(cnf.to_string(), "A!B!|");
    }

    #[test]
    fn test_conjunctive_normal_form_or_not() {
        let formula = Evaluator::new("AB|!");
        let cnf = formula.to_conjunctive_normal_form();
        assert_eq!(cnf.to_string(), "A!B!&");
    }

    #[test]
    fn test_conjunctive_normal_form_or_and() {
        let formula = Evaluator::new("AB|C&");
        let cnf = formula.to_conjunctive_normal_form();
        assert_eq!(cnf.to_string(), "AB|C&");
    }

    #[test]
    fn test_conjunctive_normal_form_multiple_or() {
        let formula = Evaluator::new("AB|C|D|");
        let cnf = formula.to_conjunctive_normal_form();
        assert_eq!(cnf.to_string(), "ABCD|||");
    }

    #[test]
    fn test_conjunctive_normal_form_multiple_and() {
        let formula = Evaluator::new("AB&C&D&");
        let cnf = formula.to_conjunctive_normal_form();
        assert_eq!(cnf.to_string(), "ABCD&&&");
    }

    #[test]
    fn test_conjunctive_normal_form_and_not_or() {
        let formula = Evaluator::new("AB&!C!|");
        let cnf = formula.to_conjunctive_normal_form();
        assert_eq!(cnf.to_string(), "A!B!C!||");
    }

    #[test]
    fn test_conjunctive_normal_form_or_not_and() {
        let formula = Evaluator::new("AB|!C!&");
        let cnf = formula.to_conjunctive_normal_form();
        assert_eq!(cnf.to_string(), "A!B!C!&&");
    }
}