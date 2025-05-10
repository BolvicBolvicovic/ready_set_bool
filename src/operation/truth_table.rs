use std::collections::{HashMap, HashSet};

use super::rpn::{
    rpn,
    rpn_format,
};

pub fn print_truth_table(formula: &str) {
    let _ = truth_table(formula, true);
}

pub fn sat(formula: &str) -> bool {
    truth_table(formula, false)
}

fn truth_table(formula: &str, print_opt: bool) -> bool {
    rpn_format(formula);

    let mut letters = formula
        .chars()
        .filter(|c| c.is_ascii_uppercase())
        .collect::<HashSet<char>>()
        .into_iter()
        .collect::<Vec<char>>();

    // Forced to not chain the sort function, because it modifies the vector in place.
    letters.sort();

    let order = letters
        .clone()
        .into_iter()
        .enumerate()
        .collect::<HashMap<usize, char>>();

    // Reverse the order so C corresponds to bit 1.
    letters.reverse();
    let letters = letters
        .into_iter()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect::<HashMap<char, usize>>();

    if print_opt {
        for i in 0..order.len() {
            print!("| {} ", order[&i]);
        }
        print!("| = |\n");
    }

    let mut sat = false;

    let max_computation = letters.len().pow(2) - (if letters.len() % 2 == 0 { 0 } else { 1 });
    for i in 0..max_computation {

        if print_opt {
            for j in 0..order.len() {
                if i & (1 << letters[&order[&j]]) != 0 {
                    print!("| 1 ");
                } else {
                    print!("| 0 ");
                }
            }
        }

        let new_formula = formula
            .chars()
            .map(|c| {
                if letters.contains_key(&c) {
                    if i & (1 << letters[&c]) != 0 {
                        '1'
                    } else {
                        '0'
                    }
                } else {
                    c
                }
            })
            .collect::<String>();
        
        let res_rpn = rpn(&new_formula);
        if res_rpn {
            sat = true;
        }
        if print_opt {
            print!("| {} |\n", if res_rpn { 1 } else { 0 });
        }
    }
    sat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sat_or() {
        assert_eq!(sat("AB|"), true);
    }

    #[test]
    fn test_sat_and() {
        print_truth_table("AB&");
        assert_eq!(sat("AB&"), true);
    }

    #[test]
    fn test_sat_not_and() {
        assert_eq!(sat("AA!&"), false);
    }

    #[test]
    fn test_sat_xor() {
        assert_eq!(sat("AA^"), false);
    }
}