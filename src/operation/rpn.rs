pub fn rpn(proposition: &str) -> bool {
    if let Some(_) = proposition.find(|c| !"01!&|^>=".contains(c)) {
        panic!("Proposition contains incorrect characters!");
    }

    proposition
        .chars()
        .into_iter()
        .fold(vec![], |mut acc, c| {
            match c {
                '0' | '1' => acc.push(c),
                '!' => {
                    let a = acc.pop().unwrap();
                    if a == '1' {
                        acc.push('0');
                    } else {
                        acc.push('1');
                    }
                },
                '&' => {
                    let b = acc.pop().unwrap();
                    let a = acc.pop().unwrap();
                    if a == b && b == '1' {
                        acc.push('1');
                    } else {
                        acc.push('0');
                    }
                },
                '|' => {
                    let b = acc.pop().unwrap();
                    let a = acc.pop().unwrap();
                    if a == '1' || b == '1' {
                        acc.push('1');
                    } else {
                        acc.push('0');
                    }
                },
                '^' => {
                    let b = acc.pop().unwrap();
                    let a = acc.pop().unwrap();
                    if a != b {
                        acc.push('1');
                    } else {
                        acc.push('0');
                    }
                },
                '>' => {
                    let b = acc.pop().unwrap();
                    let a = acc.pop().unwrap();
                    if a == '1' {
                        acc.push(b);
                    }
                },
                '=' => {
                    let b = acc.pop().unwrap();
                    let a = acc.pop().unwrap();
                    if a == b {
                        acc.push('1');
                    } else {
                        acc.push('0');
                    }
                }
                _ => panic!("Proposition contains incorrect characters!"),
            };

            acc
        })
        .first()
        .expect("Proposition contains syntax errors!") == &'1'
}