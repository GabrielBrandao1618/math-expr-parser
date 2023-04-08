#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow
}

#[derive(PartialEq, Debug, Clone)]
pub enum OperationPrimitive {
    Number { val: i64 },
    Operation { val: Box<Operation> },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Operation {
    pub a: OperationPrimitive,
    pub b: OperationPrimitive,
    pub operator: Operator,
}
