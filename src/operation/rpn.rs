pub fn rpn_format(formula: &str) {
    if let Some(_) = formula.find(|c: char| !"!&|^>=".contains(c) && !c.is_ascii_uppercase()) {
        panic!("Proposition contains incorrect characters!");
    }
}

pub fn rpn(formula: &str) -> bool {
    if let Some(_) = formula.find(|c| !"01!&|^>=".contains(c)) {
        panic!("Proposition contains incorrect characters!");
    }
    formula
        .chars()
        .into_iter()
        .fold(vec![], |mut acc, c| {
            match c {
                '0' | '1' => acc.push(if c == '1' {1} else {0}),
                '!' => {
                    let a = acc.pop().unwrap();
                    acc.push(if a == 1 {1} else {0});
                },
                '&' => {
                    let b = acc.pop().unwrap();
                    let a = acc.pop().unwrap();
                    acc.push( a & b );
                },
                '|' => {
                    let b = acc.pop().unwrap();
                    let a = acc.pop().unwrap();
                    acc.push(a | b);
                },
                '^' => {
                    let b = acc.pop().unwrap();
                    let a = acc.pop().unwrap();
                    acc.push(a ^ b);
                },
                '>' => {
                    let b = acc.pop().unwrap();
                    let a = acc.pop().unwrap();
                    if a == 1 {
                        acc.push(b);
                    }
                },
                '=' => {
                    let b = acc.pop().unwrap();
                    let a = acc.pop().unwrap();
                    acc.push(if a == b {1} else {0});
                }
                _ => panic!("Proposition contains incorrect characters!"),
            };

            acc
        })
        .first()
        .expect(&format!("Proposition contains syntax errors!\n Proposition: {formula}")) == &1
}