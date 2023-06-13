use std::collections::VecDeque;

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

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
        ];

        for (ops, expected) in testcases {
            let ans = eval(ops);
            match (expected, ans) {
                (Ok(expected), Ok(ans)) => assert_eq!(expected, ans),
                (Ok(_), Err(_)) => assert!(false),
                (Err(_), Ok(_)) => assert!(false),
                (Err(_), Err(_)) => assert!(true),
            }
        }
    }
}
