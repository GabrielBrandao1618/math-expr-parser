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
                let ast_parsed = OperationPrimitive::Number { val: parsed };
                ast_vec.push(ast_parsed);
            }
            Rule::Operation => {
                let parsed_operation = pair_to_operation(&pair);
                let operation_ast = OperationPrimitive::Operation {
                    val: Box::new(parsed_operation),
                };
                ast_vec.push(operation_ast);
            }
            _ => (),
        }
    }

    for (i, op) in ast_vec.clone().iter().enumerate() {
        if let OperationPrimitive::Operation { val } = op {
            if val.operator == Operator::Div || val.operator == Operator::Mul {
                merge_ast_vec_operation_primitives(&mut ast_vec, i);
            }
        }
    }
    for (i, op) in ast_vec.clone().iter().enumerate() {
        if let OperationPrimitive::Operation { val } = op {
            if val.operator == Operator::Add || val.operator == Operator::Sub {
                merge_ast_vec_operation_primitives(&mut ast_vec, i);
            }
        }
    }

    if let OperationPrimitive::Operation { val } = &ast_vec[0] {
        return Operation {
            a: val.a.to_owned(),
            b: val.b.to_owned(),
            operator: val.operator
        };
    } else {
        unreachable!();
    }
}

fn merge_ast_vec_operation_primitives(ast_vec: &mut Vec<OperationPrimitive>, item_index: usize) {
    let op = &ast_vec[item_index];

    if let OperationPrimitive::Operation { val } = op {
        let previous_item = &ast_vec[item_index - 1];
        let merged_operators = merge_operation_primitives(
            previous_item,
            &OperationPrimitive::Operation {
                val: val.to_owned(),
            },
        );
        ast_vec.splice(item_index - 1..item_index + 1, vec![merged_operators]);
    }
}

fn pair_to_operator<'a>(pair: &Pair<'a, Rule>) -> Operator {
    let str_operator = pair.clone().into_inner().next().unwrap().as_str();
    match str_operator {
        "+" => Operator::Add,
        "-" => Operator::Sub,
        "*" => Operator::Mul,
        "/" => Operator::Div,
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
    let parsed_num = pair_to_number(&num);
    // TODO: handle sub expressions recursively
    Operation {
        a: OperationPrimitive::Number { val: 0 },
        b: OperationPrimitive::Number { val: parsed_num },
        operator: parsed_operator,
    }
}

fn merge_operation_primitives(
    a: &OperationPrimitive,
    b: &OperationPrimitive,
) -> OperationPrimitive {
    match b {
        OperationPrimitive::Operation { val: b_op } => match a {
            OperationPrimitive::Number { val: a_num } => {
                return OperationPrimitive::Operation {
                    val: Box::new(Operation {
                        a: OperationPrimitive::Number { val: *a_num },
                        b: b_op.b.to_owned(),
                        operator: b_op.operator,
                    }),
                };
            }
            OperationPrimitive::Operation { val: a_op } => {
                return OperationPrimitive::Operation {
                    val: Box::new(Operation {
                        a: OperationPrimitive::Number { val: 0 },
                        b: OperationPrimitive::Operation {
                            val: Box::new(Operation {
                                a: a_op.b.to_owned(),
                                b: b_op.b.to_owned(),
                                operator: b_op.operator,
                            }),
                        },
                        operator: a_op.operator,
                    }),
                }
            }
        },
        OperationPrimitive::Number { .. } => panic!("b must be a operation"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_operation_primitives() {
        let a = OperationPrimitive::Number { val: 4 };
        let b = OperationPrimitive::Operation {
            val: Box::new(Operation {
                a: OperationPrimitive::Number { val: 0 },
                b: OperationPrimitive::Number { val: 10 },
                operator: Operator::Add,
            }),
        };
        let merged = merge_operation_primitives(&a, &b);
        let expected = OperationPrimitive::Operation {
            val: Box::new(Operation {
                a: OperationPrimitive::Number { val: 4 },
                b: OperationPrimitive::Number { val: 10 },
                operator: Operator::Add,
            }),
        };
        assert_eq!(merged, expected);

        let a2 = OperationPrimitive::Operation {
            val: Box::new(Operation {
                a: OperationPrimitive::Number { val: 0 },
                b: OperationPrimitive::Number { val: 4 },
                operator: Operator::Sub,
            }),
        };
        let b2 = OperationPrimitive::Operation {
            val: Box::new(Operation {
                a: OperationPrimitive::Number { val: 0 },
                b: OperationPrimitive::Number { val: 2 },
                operator: Operator::Mul,
            }),
        };
        let merged2 = merge_operation_primitives(&a2, &b2);
        let expected2 = OperationPrimitive::Operation {
            val: Box::new(Operation {
                a: OperationPrimitive::Number { val: 0 },
                b: OperationPrimitive::Operation {
                    val: Box::new(Operation {
                        a: OperationPrimitive::Number { val: 4 },
                        b: OperationPrimitive::Number { val: 2 },
                        operator: Operator::Mul,
                    }),
                },
                operator: Operator::Sub,
            }),
        };
        assert_eq!(merged2, expected2);
    }
}
