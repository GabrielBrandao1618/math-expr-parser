use pest::{iterators::Pair, Parser};

use crate::tokens::{Operation, OperationPrimitive, Operator};

#[derive(pest_derive::Parser)]
#[grammar = "parser/grammar/grammar.pest"]
pub struct ExprParser {}

pub fn parse_input(input: &str) -> Operation {
    let expr = ExprParser::parse(Rule::Expr, input)
        .unwrap()
        .next()
        .unwrap();

    let mut ast_vec: Vec<OperationPrimitive> = vec![];

    for pair in expr.clone().into_inner() {
        match pair.as_rule() {
            Rule::Number => {
                let parsed = pair_to_number(&pair);
                let ast_parsed = OperationPrimitive::Int(parsed);
                ast_vec.push(ast_parsed);
            }
            Rule::Operation => {
                let parsed_operation = pair_to_operation(&pair);
                let operation_ast = OperationPrimitive::Operation(Box::new(parsed_operation));
                ast_vec.push(operation_ast);
            }
            Rule::Expr => {
                let parsed = parse_input(pair.as_str());
                ast_vec.push(OperationPrimitive::Operation(Box::new(parsed)));
            }
            _ => unreachable!("{:#?}", pair),
        }
    }

    full_merge_ast_vec_operation_primitives(&mut ast_vec);

    if let OperationPrimitive::Operation(val) = &ast_vec[0] {
        return Operation {
            a: val.a.to_owned(),
            b: val.b.to_owned(),
            operator: val.operator,
        };
    } else {
        unreachable!("{:#?}", ast_vec);
    }
}

fn full_merge_ast_vec_operation_primitives(ast_vec: &mut Vec<OperationPrimitive>) {
    while ast_vec.len() > 1 {
        for i in 1..ast_vec.len() {
            let op = &ast_vec[i];
            if let OperationPrimitive::Operation(val) = op {
                if val.operator == Operator::Pow {
                    merge_ast_vec_operation_primitives(ast_vec, i);
                    full_merge_ast_vec_operation_primitives(ast_vec);
                    break;
                }
            }
        }
        for i in 1..ast_vec.len() {
            let op = &ast_vec[i];
            if let OperationPrimitive::Operation(val) = op {
                if val.operator == Operator::Div || val.operator == Operator::Mul {
                    merge_ast_vec_operation_primitives(ast_vec, i);
                    full_merge_ast_vec_operation_primitives(ast_vec);
                    break;
                }
            }
        }
        for i in 1..ast_vec.len() {
            let op = &ast_vec[i];
            if let OperationPrimitive::Operation(val) = op {
                if val.operator == Operator::Add || val.operator == Operator::Sub {
                    merge_ast_vec_operation_primitives(ast_vec, i);
                    full_merge_ast_vec_operation_primitives(ast_vec);
                    break;
                }
            }
        }
    }
}

fn merge_ast_vec_operation_primitives(ast_vec: &mut Vec<OperationPrimitive>, item_index: usize) {
    let op = &ast_vec[item_index];

    if let OperationPrimitive::Operation(val) = op {
        let previous_item = &ast_vec[item_index - 1];
        let merged_operators = merge_operation_primitives(
            previous_item,
            &OperationPrimitive::Operation(val.to_owned()),
        );
        ast_vec.splice(item_index - 1..item_index + 1, vec![merged_operators]);
    }
}

fn pair_to_operator<'a>(pair: &Pair<'a, Rule>) -> Operator {
    let operator = pair.clone().into_inner().next().unwrap();
    match operator.as_rule() {
        Rule::Add => Operator::Add,
        Rule::Sub => Operator::Sub,
        Rule::Mul => Operator::Mul,
        Rule::Div => Operator::Div,
        Rule::Pow => Operator::Pow,
        _ => unreachable!(),
    }
}
fn pair_to_number<'a>(pair: &Pair<'a, Rule>) -> i64 {
    match pair.as_rule() {
        Rule::Number => {
            return pair.as_str().parse().unwrap();
        }
        _ => unreachable!("{:#?}", pair.as_rule()),
    }
}

fn pair_to_operation<'a>(pair: &Pair<'a, Rule>) -> Operation {
    let mut pair_inner = pair.clone().into_inner();
    let operator = pair_inner.next().unwrap();
    let parsed_operator = pair_to_operator(&operator);

    let num = pair_inner.next().unwrap();
    match num.as_rule() {
        Rule::Number => {
            let parsed_num = pair_to_number(&num);
            return Operation {
                a: OperationPrimitive::Int(0),
                b: OperationPrimitive::Int(parsed_num),
                operator: parsed_operator,
            };
        }
        Rule::Expr => {
            let sub_operation = parse_input(num.as_str());
            return Operation {
                a: OperationPrimitive::Int(0),
                b: OperationPrimitive::Operation(Box::new(Operation {
                    a: sub_operation.a,
                    b: sub_operation.b,
                    operator: sub_operation.operator,
                })),
                operator: parsed_operator,
            };
        }
        _ => unreachable!(),
    }
}

fn merge_operation_primitives(
    a: &OperationPrimitive,
    b: &OperationPrimitive,
) -> OperationPrimitive {
    match b {
        OperationPrimitive::Operation(b_op) => match a {
            OperationPrimitive::Int(a_num) => {
                return OperationPrimitive::Operation(Box::new(Operation {
                    a: OperationPrimitive::Int(0),
                    b: OperationPrimitive::Operation(Box::new(Operation {
                        a: OperationPrimitive::Int(*a_num),
                        b: b_op.b.to_owned(),
                        operator: b_op.operator,
                    })),
                    operator: Operator::Add,
                }))
            }
            OperationPrimitive::Operation(a_op) => {
                return OperationPrimitive::Operation(Box::new(Operation {
                    a: OperationPrimitive::Int(0),
                    b: OperationPrimitive::Operation(Box::new(Operation {
                        a: a_op.b.to_owned(),
                        b: b_op.b.to_owned(),
                        operator: b_op.operator,
                    })),
                    operator: a_op.operator,
                }))
            }
        },
        OperationPrimitive::Int { .. } => panic!("b must be a operation"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_operation_primitives() {
        let a = OperationPrimitive::Int(4);
        let b = OperationPrimitive::Operation(Box::new(Operation {
            a: OperationPrimitive::Int(0),
            b: OperationPrimitive::Int(10),
            operator: Operator::Add,
        }));
        let merged = merge_operation_primitives(&a, &b);
        let expected = OperationPrimitive::Operation(Box::new(Operation {
            a: OperationPrimitive::Int(0),
            b: OperationPrimitive::Operation(Box::new(Operation {
                a: OperationPrimitive::Int(4),
                b: OperationPrimitive::Int(10),
                operator: Operator::Add,
            })),
            operator: Operator::Add,
        }));
        assert_eq!(merged, expected);

        let a2 = OperationPrimitive::Operation(Box::new(Operation {
            a: OperationPrimitive::Int(0),
            b: OperationPrimitive::Int(4),
            operator: Operator::Sub,
        }));
        let b2 = OperationPrimitive::Operation(Box::new(Operation {
            a: OperationPrimitive::Int(0),
            b: OperationPrimitive::Int(2),
            operator: Operator::Mul,
        }));
        let merged2 = merge_operation_primitives(&a2, &b2);
        let expected2 = OperationPrimitive::Operation(Box::new(Operation {
            a: OperationPrimitive::Int(0),
            b: OperationPrimitive::Operation(Box::new(Operation {
                a: OperationPrimitive::Int(4),
                b: OperationPrimitive::Int(2),
                operator: Operator::Mul,
            })),
            operator: Operator::Sub,
        }));
        assert_eq!(merged2, expected2);
    }
}
