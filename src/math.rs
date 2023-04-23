use crate::tokens::{Operation, OperationPrimitive, Operator};

pub fn resolve_operation(operation: &Operation) -> i64 {
    let a: i64;
    let b: i64;
    match &operation.a {
        OperationPrimitive::Int(val) => {
            a = *val;
        }
        OperationPrimitive::Operation(val) => {
            a = resolve_operation(&val);
        }
    }
    match &operation.b {
        OperationPrimitive::Int(val) => {
            b = *val;
        }
        OperationPrimitive::Operation(val) => {
            b = resolve_operation(&val);
        }
    }
    match operation.operator {
        Operator::Add => {
            return a + b;
        }
        Operator::Sub => {
            return a - b;
        }
        Operator::Mul => {
            return a * b;
        }
        Operator::Div => {
            return a / b;
        }
        Operator::Pow => {
            return a.pow(b as u32);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operators() {
        let mult = Operation {
            a: OperationPrimitive::Int(2),
            b: OperationPrimitive::Int(2),
            operator: Operator::Mul,
        };
        assert_eq!(resolve_operation(&mult), 4);
        let div = Operation {
            a: OperationPrimitive::Int(2),
            b: OperationPrimitive::Int(2),
            operator: Operator::Div,
        };
        assert_eq!(resolve_operation(&div), 1);
        let add = Operation {
            a: OperationPrimitive::Int(2),
            b: OperationPrimitive::Int(2),
            operator: Operator::Add,
        };
        assert_eq!(resolve_operation(&add), 4);
        let sub = Operation {
            a: OperationPrimitive::Int(2),
            b: OperationPrimitive::Int(2),
            operator: Operator::Sub,
        };
        assert_eq!(resolve_operation(&sub), 0);
    }
    #[test]
    fn test_sub_expressions() {
        let op = Operation {
            a: OperationPrimitive::Int(4),
            b: OperationPrimitive::Operation(Box::new(Operation {
                a: OperationPrimitive::Int(7),
                b: OperationPrimitive::Int(4),
                operator: Operator::Mul,
            })),
            operator: Operator::Add,
        };
        assert_eq!(resolve_operation(&op), 32);
    }

    #[test]
    fn test_power() {
        let op = Operation {
            a: OperationPrimitive::Int(2),
            b: OperationPrimitive::Int(3),
            operator: Operator::Pow,
        };
        assert_eq!(resolve_operation(&op), 8);
    }
}
