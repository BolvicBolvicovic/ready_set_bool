use std::collections::{HashMap, HashSet};

use super::rpn::{
    rpn,
    rpn_format,
};

pub fn print_truth_table(formula: &str) {
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

    for i in 0..order.len() {
        print!("| {} ", order[&i]);
    }
    print!("| = |\n");

    let max_computation = letters.len().pow(2) - 1;
    for i in 0..max_computation {

        for j in 0..order.len() {
            if i & (1 << letters[&order[&j]]) != 0 {
                print!("| 1 ");
            } else {
                print!("| 0 ");
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
        
        print!("| {} |\n", if rpn(&new_formula) { 1 } else { 0 });
    }
}