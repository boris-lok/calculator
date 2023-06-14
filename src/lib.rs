use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operator {
    Pow,
    Add,
    Sub,
    Mul,
    Div,
    LeftBucket,
    RightBucket,
}

impl Operator {
    fn get_priority(&self) -> u8 {
        match self {
            Operator::Pow => 0,
            Operator::Add => 2,
            Operator::Sub => 2,
            Operator::Mul => 1,
            Operator::Div => 1,
            Operator::LeftBucket => 3,
            Operator::RightBucket => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Ops {
    Number(i64),
    Operator(Operator),
}

#[derive(Debug)]
pub enum Error {
    InvalidExpression,
}

pub fn eval(ops: Vec<Ops>) -> Result<i64, Error> {
    let mut stack = VecDeque::new();

    for op in ops {
        match op {
            Ops::Number(number) => stack.push_back(number),
            Ops::Operator(op) => {
                let a = stack.pop_back();
                let b = stack.pop_back();

                if a.is_none() || b.is_none() {
                    return Err(Error::InvalidExpression);
                }

                let a = a.unwrap();
                let b = b.unwrap();

                match op {
                    Operator::Pow => stack.push_back(b.pow(a as u32)),
                    Operator::Add => {
                        stack.push_back(a + b);
                    }
                    Operator::Sub => {
                        stack.push_back(b - a);
                    }
                    Operator::Mul => {
                        stack.push_back(a * b);
                    }
                    Operator::Div => {
                        if a == 0 {
                            return Err(Error::InvalidExpression);
                        }
                        stack.push_back(b / a);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    let ans = stack.pop_back();

    if ans.is_none() {
        return Err(Error::InvalidExpression);
    }

    Ok(ans.unwrap())
}

pub fn organize_ops(ops: Vec<Ops>) -> Result<Vec<Ops>, Error> {
    let mut stack = Vec::new();
    let mut ans = Vec::new();

    dbg!(&ops);

    for op in ops {
        dbg!(&op, &stack);
        match op {
            Ops::Number(x) => ans.push(Ops::Number(x)),
            Ops::Operator(op) => match op {
                Operator::Div | Operator::Pow | Operator::Add | Operator::Sub | Operator::Mul => {
                    if stack.is_empty() {
                        stack.push(op.clone());
                    } else {
                        let mut c = stack.last().unwrap().clone();

                        while op.get_priority() > c.get_priority() {
                            if stack.is_empty() {
                                break;
                            }
                            c = stack.pop().unwrap();
                            ans.push(Ops::Operator(c.clone()));
                        }

                        stack.push(op.clone());
                    }
                }
                Operator::LeftBucket => stack.push(Operator::LeftBucket),
                Operator::RightBucket => loop {
                    let op = stack.pop();
                    if op.is_none() {
                        return Err(Error::InvalidExpression);
                    }
                    let op = op.unwrap();
                    if op != Operator::LeftBucket {
                        ans.push(Ops::Operator(op));
                    } else {
                        break;
                    }
                },
            },
        }
    }

    while let Some(op) = stack.pop() {
        ans.push(Ops::Operator(op));
    }

    return Ok(ans);
}

pub fn parser(expression: String) -> Result<Vec<Ops>, Error> {
    let mut format_bytes = Vec::new();

    let b = expression.bytes().collect::<Vec<_>>();
    let mut idx = 0;
    let length = b.len();

    loop {
        if idx == length {
            break;
        }

        let c = b.get(idx).unwrap();
        match c {
            &b' ' => {}
            &b'*' => format_bytes.push(Ops::Operator(Operator::Mul)),
            &b'/' => format_bytes.push(Ops::Operator(Operator::Div)),
            &b'^' => format_bytes.push(Ops::Operator(Operator::Pow)),
            &b'(' => format_bytes.push(Ops::Operator(Operator::LeftBucket)),
            &b')' => format_bytes.push(Ops::Operator(Operator::RightBucket)),
            &b'+' => {
                let next = b.get(idx + 1);
                if next.is_none() {
                    return Err(Error::InvalidExpression);
                }
                match next.unwrap() {
                    &(48..=57) => {
                        let mut number_array = vec![];
                        for tc in b[idx + 1..length].iter() {
                            match tc {
                                &(48..=57) => {
                                    number_array.push(std::char::from_u32(*tc as u32).unwrap())
                                }
                                _ => break,
                            }
                        }

                        let len = number_array.len();
                        idx += len;
                        let string_number = number_array.into_iter().collect::<String>();
                        let number = string_number.parse::<i64>().unwrap();
                        format_bytes.push(Ops::Number(number));
                    }
                    b' ' => format_bytes.push(Ops::Operator(Operator::Add)),
                    _ => unreachable!(),
                }
            }
            &b'-' => {
                let next = b.get(idx + 1);
                if next.is_none() {
                    return Err(Error::InvalidExpression);
                }
                match next.unwrap() {
                    &(48..=57) => {
                        let mut number_array = vec![];
                        for tc in b[idx + 1..length].iter() {
                            match tc {
                                &(48..=57) => {
                                    number_array.push(std::char::from_u32(*tc as u32).unwrap())
                                }
                                _ => break,
                            }
                        }

                        let len = number_array.len();
                        idx += len;
                        let string_number = number_array.into_iter().collect::<String>();
                        let number = string_number.parse::<i64>().unwrap();
                        format_bytes.push(Ops::Number(0 - number));
                    }
                    b' ' => format_bytes.push(Ops::Operator(Operator::Sub)),
                    _ => unreachable!(),
                }
            }
            c if c > &48 && c <= &57 => {
                let next = b.get(idx + 1);
                if next.is_none() {
                    let s = format!("{}", *c - 48);
                    let number = s.parse::<i64>().unwrap();
                    format_bytes.push(Ops::Number(number));
                } else {
                    match next.unwrap() {
                        &(48..=57) => {
                            let mut number_array = vec![];
                            for tc in b[idx..length].iter() {
                                match tc {
                                    &(48..=57) => {
                                        number_array.push(std::char::from_u32(*tc as u32).unwrap())
                                    }
                                    _ => break,
                                }
                            }
                            let len = number_array.len();
                            idx += len - 1;
                            let string_number = number_array.into_iter().collect::<String>();
                            let number = string_number.parse::<i64>().unwrap();
                            format_bytes.push(Ops::Number(number));
                        }
                        b' ' | b')' => {
                            let s = format!("{}", *c - 48);
                            let number = s.parse::<i64>().unwrap();
                            format_bytes.push(Ops::Number(number));
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => {
                return Err(Error::InvalidExpression);
            }
        }

        idx += 1;
    }

    return organize_ops(format_bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_works() {
        let testcases = vec![
            (
                vec![
                    Ops::Number(1),
                    Ops::Number(5),
                    Ops::Operator(Operator::Add),
                    Ops::Number(6),
                    Ops::Number(3),
                    Ops::Operator(Operator::Sub),
                    Ops::Operator(Operator::Div),
                    Ops::Number(7),
                    Ops::Operator(Operator::Mul),
                ],
                Ok::<i64, Error>(14),
            ),
            (
                vec![Ops::Number(1), Ops::Number(0), Ops::Operator(Operator::Div)],
                Err(Error::InvalidExpression),
            ),
            (
                vec![
                    Ops::Number(3),
                    Ops::Number(2),
                    Ops::Operator(Operator::Mul),
                    Ops::Number(1),
                    Ops::Operator(Operator::Add),
                ],
                Ok::<_, Error>(7),
            ),
            (
                vec![
                    Ops::Number(3),
                    Ops::Number(2),
                    Ops::Number(1),
                    Ops::Operator(Operator::Mul),
                    Ops::Operator(Operator::Add),
                ],
                Ok::<_, Error>(5),
            ),
        ];

        for (ops, expected) in testcases {
            let ans = eval(ops);
            match (expected, ans) {
                (Ok(expected), Ok(ans)) => assert_eq!(expected, ans),
                (Ok(_), Err(_)) => unreachable!(),
                (Err(_), Ok(_)) => unreachable!(),
                (Err(_), Err(_)) => continue,
            }
        }
    }

    #[test]
    fn parser_works() {
        let testcases = vec![
            (
                "11 + -3".to_string(),
                Ok::<_, Error>(vec![
                    Ops::Number(11),
                    Ops::Number(-3),
                    Ops::Operator(Operator::Add),
                ]),
            ),
            (
                "3 * 2 + 1".to_string(),
                Ok::<_, Error>(vec![
                    Ops::Number(3),
                    Ops::Number(2),
                    Ops::Operator(Operator::Mul),
                    Ops::Number(1),
                    Ops::Operator(Operator::Add),
                ]),
            ),
            (
                "3 + 2 * 1".to_string(),
                Ok::<_, Error>(vec![
                    Ops::Number(3),
                    Ops::Number(2),
                    Ops::Number(1),
                    Ops::Operator(Operator::Mul),
                    Ops::Operator(Operator::Add),
                ]),
            ),
            (
                "(3 + 2) * 1".to_string(),
                Ok::<_, Error>(vec![
                    Ops::Number(3),
                    Ops::Number(2),
                    Ops::Operator(Operator::Add),
                    Ops::Number(1),
                    Ops::Operator(Operator::Mul),
                ]),
            ),
            (
                "(3 + 2 * 4) * 1".to_string(),
                Ok::<_, Error>(vec![
                    Ops::Number(3),
                    Ops::Number(2),
                    Ops::Number(4),
                    Ops::Operator(Operator::Mul),
                    Ops::Operator(Operator::Add),
                    Ops::Number(1),
                    Ops::Operator(Operator::Mul),
                ]),
            ),
            (
                "((1)) * 1".to_string(),
                Ok::<_, Error>(vec![
                    Ops::Number(1),
                    Ops::Number(1),
                    Ops::Operator(Operator::Mul),
                ]),
            ),
        ];

        for (expression, expected) in testcases {
            let ans = parser(expression);
            match (expected, ans) {
                (Ok(expected), Ok(ans)) => assert_eq!(expected, ans),
                (Ok(_), Err(_)) => unreachable!(),
                (Err(_), Ok(_)) => unreachable!(),
                (Err(_), Err(_)) => continue,
            }
        }
    }
}
