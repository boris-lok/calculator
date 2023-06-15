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

impl From<&u8> for Operator {
    fn from(value: &u8) -> Self {
        match value {
            &b'^' => Operator::Pow,
            &b'+' => Operator::Add,
            &b'-' => Operator::Sub,
            &b'*' => Operator::Mul,
            &b'/' => Operator::Div,
            &b'(' => Operator::LeftBucket,
            &b')' => Operator::RightBucket,
            _ => unimplemented!(),
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

// eval the operators list to calculate the result
// by using stack calculation.
// 2 3 + 1 * -> 5 1 * -> 5
pub fn eval(ops: Vec<Ops>) -> Result<i64, Error> {
    let mut stack = VecDeque::new();

    for op in ops {
        match op {
            Ops::Number(number) => stack.push_back(number),
            Ops::Operator(op) => {
                if let (Some(a), Some(b)) = (stack.pop_back(), stack.pop_back()) {
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
                } else {
                    return Err(Error::InvalidExpression);
                }
            }
        }
    }

    match stack.pop_back() {
        Some(ans) => Ok(ans),
        None => Err(Error::InvalidExpression),
    }
}

// organize the operators to postfix expression
// 2 + 3 * 2 -> 2 3 2 * +
pub fn organize_ops(ops: Vec<Ops>) -> Result<Vec<Ops>, Error> {
    let mut stack: Vec<Operator> = Vec::new();
    let mut ans = Vec::new();

    for op in ops {
        match op {
            Ops::Number(x) => ans.push(Ops::Number(x)),
            Ops::Operator(op) => match op {
                Operator::Div | Operator::Pow | Operator::Add | Operator::Sub | Operator::Mul => {
                    if !stack.is_empty() {
                        let mut c = stack.last().unwrap().clone();

                        while !stack.is_empty() && op.get_priority() > c.get_priority() {
                            c = stack.pop().unwrap();
                            ans.push(Ops::Operator(c.clone()));
                        }
                    }

                    stack.push(op.clone());
                }
                Operator::LeftBucket => stack.push(Operator::LeftBucket),
                Operator::RightBucket => 'outer: loop {
                    match stack.pop() {
                        Some(op) => {
                            if op != Operator::LeftBucket {
                                ans.push(Ops::Operator(op));
                            } else {
                                break 'outer;
                            }
                        }
                        None => {
                            return Err(Error::InvalidExpression);
                        }
                    };
                },
            },
        }
    }

    Ok(ans
        .into_iter()
        .chain(stack.into_iter().rev().map(|op| Ops::Operator(op)))
        .collect())
}

// get the continuous number from the vector<u8>
// +231 / -231 / 123
fn get_number(data: &[u8]) -> Vec<char> {
    data.iter()
        .take_while(|b| **b > 48 && **b <= 57)
        .map(|b| std::char::from_u32(*b as u32).unwrap())
        .collect()
}

// transfer a vector<char> to an number
fn bytes_to_number(data: &[char]) -> i64 {
    let string_number = data.into_iter().collect::<String>();
    string_number.parse::<i64>().unwrap()
}

// parse the string expression to postfix expression
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
            c if c == &b'*' || c == &b'/' || c == &b'^' || c == &b'(' || c == &b')' => {
                format_bytes.push(Ops::Operator(c.into()))
            }
            c if c == &b'+' || c == &b'-' => {
                let add_or_sub: Operator = c.into();
                let next = b.get(idx + 1);
                if next.is_none() {
                    return Err(Error::InvalidExpression);
                }
                match next.unwrap() {
                    &(48..=57) => {
                        let number_array = get_number(&b[idx + 1..length]);
                        idx += number_array.len();
                        let number = bytes_to_number(&number_array);
                        format_bytes.push(Ops::Number(if add_or_sub == Operator::Sub {
                            0 - number
                        } else {
                            number
                        }));
                    }
                    b' ' => format_bytes.push(Ops::Operator(add_or_sub)),
                    _ => unreachable!(),
                }
            }
            c if c > &48 && c <= &57 => {
                let number_array = get_number(&b[idx..length]);
                idx += number_array.len() - 1;
                let number = bytes_to_number(&number_array);
                format_bytes.push(Ops::Number(number));
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
            (
                vec![
                    Ops::Number(3),
                    Ops::Number(2),
                    Ops::Number(4),
                    Ops::Operator(Operator::Mul),
                    Ops::Operator(Operator::Add),
                    Ops::Number(1),
                    Ops::Operator(Operator::Mul),
                ],
                Ok::<_, Error>(11),
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
