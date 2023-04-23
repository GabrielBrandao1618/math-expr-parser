use crate::tokens::{Operation, OperationPrimitive, Operator};

pub fn resolve_operation(operation: &Operation) -> f64 {
    let a: f64;
    let b: f64;
    match &operation.a {
        OperationPrimitive::Number(val) => {
            a = *val;
        }
        OperationPrimitive::Operation(val) => {
            a = resolve_operation(&val);
        }
    }
    match &operation.b {
        OperationPrimitive::Number(val) => {
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
            return a.powf(b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operators() {
        let mult = Operation {
            a: OperationPrimitive::Number(2.0),
            b: OperationPrimitive::Number(2.0),
            operator: Operator::Mul,
        };
        assert_eq!(resolve_operation(&mult), 4.0);
        let div = Operation {
            a: OperationPrimitive::Number(2.0),
            b: OperationPrimitive::Number(2.0),
            operator: Operator::Div,
        };
        assert_eq!(resolve_operation(&div), 1.0);
        let add = Operation {
            a: OperationPrimitive::Number(2.0),
            b: OperationPrimitive::Number(2.0),
            operator: Operator::Add,
        };
        assert_eq!(resolve_operation(&add), 4.0);
        let sub = Operation {
            a: OperationPrimitive::Number(2.0),
            b: OperationPrimitive::Number(2.0),
            operator: Operator::Sub,
        };
        assert_eq!(resolve_operation(&sub), 0.0);
    }
    #[test]
    fn test_sub_expressions() {
        let op = Operation {
            a: OperationPrimitive::Number(4.0),
            b: OperationPrimitive::Operation(Box::new(Operation {
                a: OperationPrimitive::Number(7.0),
                b: OperationPrimitive::Number(4.0),
                operator: Operator::Mul,
            })),
            operator: Operator::Add,
        };
        assert_eq!(resolve_operation(&op), 32.0);
    }

    #[test]
    fn test_power() {
        let op = Operation {
            a: OperationPrimitive::Number(2.0),
            b: OperationPrimitive::Number(3.0),
            operator: Operator::Pow,
        };
        assert_eq!(resolve_operation(&op), 8.0);
    }
}
