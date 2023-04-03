#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
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
